import os.path
from contextlib import contextmanager
from dataclasses import dataclass
from textwrap import dedent

from .. import schema

from .base import BaseGenerator, Type, Method, ANY_TYPE_FALLBACK
from ._utils import get_resource, pascal_ident, sanitize_path, snake_ident, words
from ..schema import parse_path_component


def _escape_doc(doc: str) -> str:
    return doc.replace("\\", "\\\\").replace('"', '\\"').replace("\n", " ")


def _escape_ident(ident: str) -> str:
    reserved = {"macro", "type", "in", "ref", "let"}
    return f"r#{ident}" if ident in reserved else ident


def _format_annotation_kwargs(kwargs: dict[str, str]) -> str:
    formatted = []
    for k, v in kwargs.items():
        formatted.append(f"{k}={v}" if v else k)
    return ", ".join(formatted)


def _as_rust_type(type: Type) -> str:
    def base_type_name():
        match type.name:
            case "string":
                return "String"
            case "integer":
                return "u64"
            case "number":
                return "f64"
            case "boolean":
                return "bool"
            case "array":
                return f"Vec<{_as_rust_type(type.element_type)}>"
            case "null":
                return "()"
            case t if t == ANY_TYPE_FALLBACK:
                return "serde_json::Value"
            case _:
                return type.name

    name = base_type_name()

    if type.optional:
        name = f"Option<{name}>"

    return name


@dataclass
class Generator(BaseGenerator):
    output_directory: str = "./rust"
    async_: bool = False

    @contextmanager
    def struct(self, name: str, generics: list[str] = None, constraints: list[str] = None):
        if generics:
            name = f"{name}<{', '.join(generics)}>"
        try:
            self.output(f"pub struct {name} {{")
            with self.indented():
                yield
        finally:
            self.output(f"}}")

    @contextmanager
    def impl(self, name: str, generics: list[str] = None, constraints: list[str] = None):
        formatted_generics = ""
        if generics:
            formatted_generics = f"<{', '.join(generics)}>"

        formatted_constraints = ""
        if constraints:
            formatted_constraints = f"where {', '.join(constraints)}"
        try:
            self.output(f"impl{formatted_generics} {name}{formatted_generics} {formatted_constraints}{{")
            with self.indented():
                yield
        finally:
            self.output(f"}}")

    @contextmanager
    def output_method(self, method: Method, async_: bool = False):
        if method.doc is not None:
            self.output(f'#[doc = "{_escape_doc(method.doc)}"]')

        fmtd_parameters = ["&self"] if not method.static else []
        fmtd_parameters += [f"{name}: {_as_rust_type(type)}" for name, type in method.parameters]
        fmtd_return = f" -> {_as_rust_type(method.return_type)}" if method.return_type is not None else ""
        fmtd_async = f"async" if async_ else ""

        self.output(f"pub {fmtd_async} fn {_escape_ident(method.name)}({', '.join(fmtd_parameters)}){fmtd_return} {{")
        with self.indented():
            yield
        self.output("}")
        self.output_newline()

    def output_type(self, type: Type):
        if type.doc is not None:
            self.output(f'#[doc = "{_escape_doc(type.doc)}"]')

        self.output("#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]")
        with self.struct(type.name):
            for k, v in type.attributes:
                if v.doc is not None:
                    self.output(f'#[doc = "{_escape_doc(v.doc)}"]')

                serde_opts = {}
                if v.original_name is not None and v.original_name != v.name:
                    serde_opts["rename"] = f'"{v.original_name}"'
                if v.type.optional and not v.repeated:
                    serde_opts["skip_serializing_if"] = '"Option::is_none"'
                    serde_opts["default"] = None
                if v.type.name == "boolean":
                    if v.type.optional:
                        serde_opts["deserialize_with"] = '"crate::gen::common::deserialize_option_bool_lax"'
                        serde_opts["serialize_with"] = '"crate::gen::common::serialize_option_bool_as_u64"'
                    else:
                        serde_opts["deserialize_with"] = '"crate::gen::common::deserialize_bool_lax"'
                        serde_opts["serialize_with"] = '"crate::gen::common::serialize_bool_as_u64"'
                if serde_opts:
                    self.output(f"#[serde({_format_annotation_kwargs(serde_opts)})]")

                rust_type = _as_rust_type(v.type)
                if v.repeated:
                    rust_type = f"std::collections::HashMap<u32, {rust_type}>"
                self.output(f"pub {_escape_ident(v.name)}: {rust_type},")

    def output_new_method(self, node: schema.Node):
        name, is_variable = parse_path_component(node.text)

        method = Method(
            name="new",
            parameters=[("client", Type(name="T"))],
            return_type=Type(name="Self"),
            static=True,
        )
        is_root_element = node.path.count("/") == 1 and node.path[0] == "/"
        if not is_root_element:
            method.parameters.append(("parent_path", Type(name="&str")))

        param = f'"{node.text}"'
        if is_variable:
            method.parameters.append((name, Type(name="&str")))
            param = name

        with self.output_method(method):
            if is_root_element:
                self.output(
                    dedent(
                        f"""\
                    Self {{
                        client,
                        path: {param}.to_string(),
                    }}
                    """
                    )
                )
            else:
                self.output(
                    dedent(
                        f"""\
                    Self {{
                        client,
                        path: format!("{{}}/{{}}", parent_path, {param}),
                    }}
                    """
                    )
                )

    def output_client_type(self, struct: str, node, methods: list[Method], async_: bool = False):
        client_trait = "crate::client::HttpClient"
        if async_:
            struct = f"Async{struct}"
            client_trait = "crate::client::AsyncHttpClient"

        # Generate struct definition
        self.output(f"#[derive(Debug, Clone)]")
        with self.struct(struct, generics=["T"]):
            self.output("client: T,")
            self.output("path: String,")

        self.output_newline()

        # Create impl block
        with self.impl(struct, generics=["T"], constraints=["T: Clone"]):
            self.output_new_method(node)

            # Create methods to obtain child clients
            if node.children is not None:
                for child in node.children:
                    child_name, is_variable = parse_path_component(child.text)
                    ret = f"{_escape_ident(snake_ident(child_name))}::{pascal_ident(child_name, 'Client')}"
                    method = Method(
                        name=snake_ident(child_name),
                        return_type=Type(name=f"{ret}<T>"),
                        parameters=[(child_name, Type(name="&str"))] if is_variable else [],
                    )

                    with self.output_method(method):
                        if is_variable:
                            self.output(
                                dedent(
                                    f"""\
                                    {ret}::<T>::new(
                                        self.client.clone(),
                                        &self.path,
                                        {child_name},
                                    )
                                    """
                                )
                            )
                        else:
                            self.output(
                                dedent(
                                    f"""\
                                    {ret}::<T>::new(
                                        self.client.clone(),
                                        &self.path,
                                    )
                                    """
                                )
                            )

        with self.impl(struct, generics=["T"], constraints=[f"T: {client_trait}"]):
            # Create methods for HTTP operations
            for m in methods:
                with self.output_method(m, async_=async_):
                    suffix = ""
                    if async_:
                        suffix = ".await"

                    fmtd_parameters = "&()"
                    if m.parameters:
                        fmtd_parameters = ", ".join(f"&{name}" for name, _ in m.parameters)
                    self.output(f"self.client.{m.name}(&self.path, {fmtd_parameters}){suffix}")

    def process(self, root: list[schema.Node]):
        # Create "root" file declaring all child modules
        with self.file(f"{self.output_directory}/../{os.path.basename(self.output_directory)}.rs"):
            for node in root:
                self.output(f"pub mod {node.path.strip('/')};")
            self.output_newline()
            self.output("pub mod common;")

        # Generate custom types
        with open(f"{self.output_directory}/common.rs", "w") as f:
            f.write(get_resource("common.rs"))

        for node in schema.traverse(root):
            info, path = node.info, node.path

            filepath = sanitize_path(path)
            full_path = os.path.join(self.output_directory, f"{filepath}.rs".lstrip("/"))

            mods = []
            if node.children is not None:
                mods = [child.text.lstrip("{").rstrip("}") for child in node.children]

            with self.file(full_path):
                if len(mods):
                    for mod in sorted(mods):
                        self.output(f"pub mod {_escape_ident(snake_ident(mod))};")
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

                                m.return_type = Type(name=f"Result<{_as_rust_type(m.return_type)}, T::Error>")

                            m.can_support_async = self.async_
                            methods.append(m)
                        except ValueError as e:
                            print(f"{full_path}: {struct}: unable to generate method for '{v.method}': {e}")

                # Create extra types (for parameters and return types)
                for type in types:
                    self.output_type(type)
                    self.output_newline()

                self.output_client_type(struct, node, methods)
                if self.async_:
                    self.output_client_type(struct, node, methods, async_=True)
