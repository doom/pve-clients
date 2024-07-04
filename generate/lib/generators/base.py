import os
from contextlib import contextmanager
from dataclasses import dataclass, field
from typing import TextIO, Optional

from lib import schema
from lib.generators._utils import pascal_ident, snake_ident

from lib.schema import Node, is_variable


class BaseGenerator:
    writer: TextIO = None
    indent: int = 0
    indent_offset: int = 4

    def output(self, *args: str):
        for arg in args:
            for line in arg.splitlines():
                print(self.indent * " ", end="", file=self.writer)
                print(line, file=self.writer)

    def output_newline(self):
        print(self.indent * " ", end="", file=self.writer)
        print("", file=self.writer)

    @contextmanager
    def indented(self):
        self.indent += self.indent_offset
        try:
            yield
        finally:
            self.indent -= self.indent_offset

    @contextmanager
    def file(self, path: str):
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w") as f:
            self.writer = f
            self.indent = 0
            yield

    def process(self, schema: list[Node]):
        pass


ANY_TYPE_FALLBACK = "anytype"


@dataclass
class Type:
    name: str
    doc: Optional[str] = None
    attributes: list[tuple[str, "Attribute"]] = None
    optional: bool = False
    allowed_values: Optional[list] = None
    element_type: Optional["Type"] = None

    @classmethod
    def from_schema_parameters(cls, name: str, parameters: schema.Parameters, *, path: str) -> "Type":
        attributes = []
        if parameters.properties is not None:
            path_vars = {name for name, is_var in (schema.parse_path_component(c) for c in path.split("/")) if is_var}

            for k, v in parameters.properties.items():
                # Create an attribute only if the parameter is not already given in the path
                if k not in path_vars:
                    attributes.append((k, Attribute.from_schema(k, v, suggested_type_name=pascal_ident(name, k))))

        return cls(
            name=name,
            attributes=attributes,
        )

    @classmethod
    def from_schema(cls, type: schema.Type, suggested_name: str = None) -> "Type":
        name = type.type

        if name in {"unknown", "any"}:
            if type.format is None:
                name = ANY_TYPE_FALLBACK
            else:
                name = "string"

        if name == "object" and suggested_name is not None:
            name = suggested_name

        attributes = None
        if isinstance(type, schema.ObjectType):
            attributes = []
            if type.properties is not None:
                for k, v in type.properties.items():
                    if is_variable(k):
                        continue  # TODO: handle this case, seems to only occur in cluster/ceph/metadata

                    attributes.append((k, Attribute.from_schema(k, v, suggested_type_name=pascal_ident(name, k))))

        element_type = None
        if isinstance(type, schema.ArrayType):
            if type.items is None:
                element_type = Type(name=ANY_TYPE_FALLBACK)
            else:
                element_type = cls.from_schema(type.items, suggested_name=pascal_ident(suggested_name, "Item"))

        return cls(
            name=name,
            doc=type.description,
            optional=type.optional is not None and type.optional == 1,
            allowed_values=type.enum,
            attributes=attributes,
            element_type=element_type,
        )

    def is_primitive(self) -> bool:
        return self.name in {"string", "number", "integer", "boolean", "array", "null", ANY_TYPE_FALLBACK}

    def is_array(self) -> bool:
        return self.name == "array"

    def iter_nested_types(self):
        if self.attributes is not None:
            for _, attr in self.attributes:
                if not attr.type.is_primitive():
                    yield attr.type
                yield from attr.type.iter_nested_types()
        elif self.is_array():
            if not self.element_type.is_primitive():
                yield self.element_type
            yield from self.element_type.iter_nested_types()


@dataclass
class Attribute:
    name: str
    type: Type
    doc: Optional[str] = None
    repeated: bool = False
    original_name: Optional[str] = None

    @classmethod
    def from_schema(cls, name: str, type: schema.Type, suggested_type_name: str = None) -> "Attribute":
        original_name = name

        repeated = False
        if name.endswith("[n]"):
            name = name[:-3] + "s"
            repeated = True

        name = snake_ident(name)

        attr_type = Type.from_schema(
            type, suggested_name=pascal_ident(suggested_type_name) if suggested_type_name is not None else None
        )
        if not attr_type.is_primitive():
            attr_type.name = pascal_ident(name)

        return cls(
            name=name,
            type=attr_type,
            doc=type.description,
            repeated=repeated,
            original_name=original_name,
        )


@dataclass
class Method:
    name: str
    doc: Optional[str] = None
    return_type: Optional[Type] = None
    parameters: list[tuple[str, Type]] = field(default_factory=list)
    static: bool = False
    can_support_async: bool = False

    @classmethod
    def from_schema(cls, method_info: schema.MethodInfo, *, path: str) -> "Method":
        name = snake_ident(method_info.method)
        parameters = []
        if (ps := method_info.parameters) is not None:
            if (
                ps.additional_properties is not None
                and (not isinstance(ps.additional_properties, int) or ps.additional_properties > 0)
            ) or (ps.properties is not None and len(ps.properties) > 0):
                parameters_type = Type.from_schema_parameters(pascal_ident(name, "Parameters"), ps, path=path)

                # Generate the Parameters type only if it is not empty
                if parameters_type.attributes:
                    parameters = [("parameters", parameters_type)]

        return_type = None
        if method_info.returns is not None:
            return_type = Type.from_schema(
                method_info.returns,
                suggested_name=pascal_ident(method_info.method, "Response"),
            )

        return cls(
            name=name,
            doc=method_info.description,
            return_type=return_type,
            parameters=parameters,
        )
