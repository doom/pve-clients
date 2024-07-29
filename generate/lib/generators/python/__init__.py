import os.path
from contextlib import contextmanager
from dataclasses import dataclass
from textwrap import dedent

from ... import schema

from ..base import BaseGenerator, Type, Method, ANY_TYPE_FALLBACK
from .._utils import get_resource, pascal_ident, sanitize_path, snake_ident, words


def _escape_doc(doc: str) -> str:
    return doc.replace("\\", "\\\\").replace('"', '\\"').replace("\n", " ")


def _escape_ident(ident: str) -> str:
    reserved = {"def", "for", "in", "class", "list"}
    return f"{ident}_" if ident in reserved else ident


def _escape_path(path: str) -> str:
    return "/".join(_escape_ident(comp) for comp in path.split("/"))


def _format_annotation_kwargs(kwargs: dict[str, str]) -> str:
    formatted = []
    for k, v in kwargs.items():
        formatted.append(f"{k}={v}" if v else k)
    return ", ".join(formatted)


def _as_python_type(type: Type) -> str:
    def base_type_name():
        match type.name:
            case "string":
                return "str"
            case "integer":
                return "int"
            case "number":
                return "float"
            case "boolean":
                return "bool"
            case "array":
                return f"list[{_as_python_type(type.element_type)}]"
            case "null":
                return "None"
            case t if t == ANY_TYPE_FALLBACK:
                return "dict[str, Any]"
            case _:
                return type.name

    name = base_type_name()

    if type.optional:
        name = f"Optional[{name}]"

    return name


@dataclass
class Generator(BaseGenerator):
    output_directory: str = "./python"
    async_: bool = False
    root_name: str = "pve"

    @contextmanager
    def struct(self, name: str, bases: list[str] = None):
        fmted_bases = ""
        if bases:
            fmted_bases = "(" + ", ".join(bases) + ")"

        try:
            self.output(f"class {name}{fmted_bases}:")
            with self.indented():
                yield
        finally:
            pass

    @contextmanager
    def output_method(self, method: Method, async_: bool = False):
        fmtd_parameters = ["self"] if not method.static else []
        fmtd_parameters += [f"{name}: {_as_python_type(type)}" for name, type in method.parameters]
        fmtd_return = f" -> {_as_python_type(method.return_type)}" if method.has_return_type() else ""
        fmtd_async = f"async " if async_ else ""

        self.output(f"{fmtd_async}def {_escape_ident(method.name)}({', '.join(fmtd_parameters)}){fmtd_return}:")
        with self.indented():
            if method.doc:
                self.output(f'"""\n{_escape_doc(method.doc)}\n"""')
            yield
        self.output_newline()

    def output_type(self, type: Type):
        with self.struct(type.name, bases=["BaseModel"]):
            extra_snippets = []

            if type.doc:
                self.output(f'"""\n{type.doc}\n"""')

            if len(type.attributes) == 0:
                self.output("pass")
                return

            model_serializer_hooks = []
            for k, v in type.attributes:
                if v.doc:
                    self.output(f"# {_escape_doc(v.doc)}")

                name = _escape_ident(v.name)

                pydantic_opts = {}
                if _escape_ident(v.name) != v.name:
                    pydantic_opts["alias"] = f'"{v.name}"'
                if v.original_name is not None and v.original_name != v.name:
                    pydantic_opts["alias"] = f'"{v.original_name}"'
                if v.type.optional:
                    pydantic_opts["default"] = "None"
                if v.repeated:
                    prefix = k.removesuffix("[n]")
                    del pydantic_opts["alias"]
                    extra_snippets.append(
                        dedent(
                            f"""\
                    @model_validator(mode='before')
                    @classmethod
                    def rewrite_for_{name}(cls, data: Any) -> Any:
                        if isinstance(data, dict):
                            data = extract_repeated_with_prefix(data, group="{name}", prefix="{prefix}")
                        return data
                    """
                        )
                    )
                    model_serializer_hooks.append(
                        f'serialize_repeated_with_prefix(data, group="{name}", prefix="{prefix}")'
                    )
                field_attr = ""
                if pydantic_opts:
                    field_attr = f"= Field({_format_annotation_kwargs(pydantic_opts)})"

                python_type = _as_python_type(v.type)
                if v.repeated:
                    python_type = f"dict[int, {python_type}]"
                self.output(f"{name}: {python_type} {field_attr}")

            if model_serializer_hooks:
                newline = "\n" + "    " * 6
                extra_snippets.append(
                    dedent(
                        f"""\
                    @model_serializer(mode="wrap")
                    def serialize(self, serializer):
                        data = serializer(self)
                        {newline.join(('data = ' + h) for h in model_serializer_hooks)}
                        return data
                """
                    )
                )

            extra_snippets.append(
                dedent(
                    f"""\
            class Config(CommonPydanticConfig):
                pass
            """
                )
            )

            self.output_newline()
            for s in extra_snippets:
                self.output(s)
                self.output_newline()

    def output_new_method(self, node: schema.Node, async_: bool = False):
        name, is_variable = schema.parse_path_component(node.text)

        method = Method(
            name="__init__",
            parameters=[("client", Type(name="AbstractClient" if not async_ else "AsyncAbstractClient"))],
        )
        is_root_element = node.path.count("/") == 1 and node.path[0] == "/"
        if not is_root_element:
            method.parameters.append(("parent_path", Type(name="str")))

        param = f"'{node.text}'"
        if is_variable:
            method.parameters.append((name, Type(name="str")))
            param = name

        with self.output_method(method):
            self.output("self.client = client")
            if is_root_element:
                self.output(f"self.path = {param}")
            else:
                self.output(f'self.path = f"{{parent_path}}/{{{param}}}"')

    def output_client_type(self, struct: str, node, methods: list[Method], async_: bool = False):
        client_protocol = "AbstractClient"
        if async_:
            struct = f"Async{struct}"
            client_protocol = f"Async{client_protocol}"

        # Generate struct definition
        self.output("@dataclass")
        with self.struct(struct):
            self.output(f"client: {client_protocol}")
            self.output("path: str")

            self.output_newline()

            # Create impl block
            self.output_new_method(node, async_=async_)

            # Create methods to obtain child clients
            if node.children is not None:
                for child in node.children:
                    child_name, is_variable = schema.parse_path_component(child.text)
                    child_client_class = pascal_ident(child_name, "Client")
                    if async_:
                        child_client_class = f"Async{child_client_class}"
                    ret = f"_{_escape_ident(snake_ident(child_name))}.{child_client_class}"
                    method = Method(
                        name=snake_ident(child_name),
                        return_type=Type(name=f"{ret}"),
                        parameters=[(child_name, Type(name="str"))] if is_variable else [],
                    )

                    with self.output_method(method):
                        if is_variable:
                            self.output(
                                dedent(
                                    f"""\
                                    return {ret}(
                                        self.client,
                                        self.path,
                                        {child_name},
                                    )
                                    """
                                )
                            )
                        else:
                            self.output(
                                dedent(
                                    f"""\
                                    return {ret}(
                                        self.client,
                                        self.path,
                                    )
                                    """
                                )
                            )

            # Create methods for HTTP operations
            for m in methods:
                with self.output_method(m, async_=async_):
                    prefix = ""
                    if async_:
                        prefix = "await "

                    params = [name for name, _ in m.parameters]
                    if m.has_return_type():
                        params.append(f"parse_as={_as_python_type(m.return_type)}")
                    fmtd_parameters = ", ".join(name for name in params)
                    self.output(f"return {prefix}self.client.{m.name}(self.path, {fmtd_parameters})")

    def process(self, root: list[schema.Node]):
        os.makedirs(self.output_directory, exist_ok=True)

        # Generate pre-written code
        files = {"client.py", "common.py"}
        for file in files:
            with open(f"{self.output_directory}/{file}", "w") as f:
                f.write(get_resource("python", file))

        for node in schema.traverse(root):
            info, path = node.info, node.path

            filepath = _escape_path(sanitize_path(path))
            full_path = os.path.join(self.output_directory, f"{filepath}/__init__.py".lstrip("/"))

            mods = []
            if node.children is not None:
                mods = [child.text.lstrip("{").rstrip("}") for child in node.children]

            with self.file(full_path):
                self.output("from dataclasses import dataclass")
                self.output("from typing import Any, Optional")
                self.output_newline()
                self.output("from pydantic import BaseModel, Field, model_serializer, model_validator")
                self.output_newline()
                self.output(f"from {self.root_name}.client import AbstractClient, AsyncAbstractClient")
                self.output(
                    f"from {self.root_name}.common import CommonPydanticConfig, extract_repeated_with_prefix, serialize_repeated_with_prefix"
                )
                if len(mods):
                    for mod in sorted(mods):
                        ident = _escape_ident(snake_ident(mod))
                        self.output(f"from . import {ident} as _{ident}")
                    self.output_newline()
                self.output_newline()

                struct = pascal_ident(*words(node.text), "client")

                methods = []
                types = []
                if info is not None:
                    for k, v in info.items():
                        try:
                            m = Method.from_schema(v, path=node.path)
                            for _, type in m.parameters:
                                if not type.is_primitive():
                                    types.append(type)
                                    types.extend(type.iter_nested_types())

                            if (return_type := m.return_type) is not None:
                                if return_type.is_array():
                                    return_type = return_type.element_type
                                if not return_type.is_primitive():
                                    return_type.name = pascal_ident(m.name, "Response", "Item")
                                    types.append(return_type)
                                    types.extend(return_type.iter_nested_types())

                            m.can_support_async = self.async_
                            methods.append(m)
                        except ValueError as e:
                            print(f"{full_path}: {struct}: unable to generate method for '{v.method}': {e}")

                # Create extra types (for parameters and return types)
                # This is done in reversed order because Python does not support forward declarations
                for type in reversed(types):
                    self.output_type(type)
                    self.output_newline()

                self.output_client_type(struct, node, methods)
                if self.async_:
                    self.output_client_type(struct, node, methods, async_=True)
