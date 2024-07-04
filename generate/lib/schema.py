import re
from typing import Annotated, Any, Generator, Literal, Optional, Union

from pydantic import BaseModel, Field, BeforeValidator

Flag = int

PATHVAR_REGEXP = re.compile(r"{(.+)}")


class _TypeBase(BaseModel):
    alias: Optional[str] = None  # Indicates this field is an alias of another field in the type
    default_key: Optional[int] = None  # ???
    description: Optional[str] = None
    enum: Optional[list] = None
    format: Optional[str | dict] = None
    format_description: Optional[str] = None
    key_alias: Optional[str] = Field(alias="keyAlias", default=None)  # ???
    optional: Optional[Flag] = 0
    renderer: Optional[str] = None
    requires: Optional[str] = None  # Indicates a requirement over another field in the type
    type_text: Optional[str] = Field(alias="typetext", default=None)  # Human-friendly version of the type ?
    verbose_description: Optional[str] = None


class StringType(_TypeBase):
    type: Literal["string"]
    min_length: Optional[int] = Field(alias="minLength", default=None)
    max_length: Optional[int] = Field(alias="maxLength", default=None)
    pattern: Optional[str] = None
    default: Optional[str] = None

    enum: Optional[list[str]] = None  # Enums are also strings


class BooleanType(_TypeBase):
    type: Literal["boolean"]
    default: Optional[bool | str] = None


class NumberType(_TypeBase):
    type: Literal["number"]
    min: Optional[int] = None
    max: Optional[int] = None


class IntegerType(_TypeBase):
    type: Literal["integer"]
    min: Optional[int] = None
    max: Optional[int] = None


class ArrayType(_TypeBase):
    type: Literal["array"]
    items: Optional["Type"] = None


class ObjectType(_TypeBase):
    type: Literal["object"]
    additional_properties: Optional[Union[Flag, "Type"]] = Field(alias="additionalProperties", default=None)
    properties: Optional[dict[str, "Type"]] = None


class NullType(_TypeBase):
    type: Literal["null"]


class AnyType(_TypeBase):
    type: Literal["any"]


class UnknownType(_TypeBase):
    type: Literal["unknown"]


def _set_type_if_missing(data: Any) -> Any:
    if isinstance(data, dict) and "type" not in data:
        if "properties" in data:
            data["type"] = "object"
        else:
            data["type"] = "unknown"
    return data


Type = Annotated[
    StringType | BooleanType | IntegerType | NumberType | ArrayType | ObjectType | NullType | AnyType | UnknownType,
    Field(discriminator="type"),
    BeforeValidator(_set_type_if_missing),
]


class Parameters(BaseModel):
    additional_properties: Optional[Flag | Type] = Field(alias="additionalProperties", default=None)
    properties: Optional[dict[str, Type]] = None
    type: Optional[str] = None


class Permissions(BaseModel):
    check: Optional[list] = None
    description: Optional[str] = None
    user: Optional[str] = None


class MethodInfo(BaseModel):
    allow_token: Flag = Field(alias="allowtoken")
    method: str
    name: str
    description: Optional[str] = None
    parameters: Optional[Parameters] = None
    permissions: Optional[Permissions] = None
    protected: Optional[Flag] = None
    proxy_to: Optional[str] = Field(alias="proxyto", default=None)
    returns: Optional[Type] = None


NodeInfo = dict[Literal["GET", "POST", "PUT", "DELETE"], MethodInfo]


class Node(BaseModel):
    path: str
    leaf: Flag
    info: Optional[NodeInfo] = None
    text: Optional[str] = None
    children: Optional[list["Node"]] = None


def traverse(node: list[Node] | Node) -> Generator[Node, None, None]:
    if isinstance(node, list):
        for child in node:
            yield from traverse(child)
    else:
        yield node
        if node.children is not None:
            yield from traverse(node.children)


def parse_path_component(component: str) -> tuple[str, bool]:
    if m := PATHVAR_REGEXP.fullmatch(component):
        return m.group(1), True

    return component, False


def is_variable(ident: str) -> bool:
    _, v = parse_path_component(ident)
    return v
