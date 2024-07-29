from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import name as _name


class PostParameters(BaseModel):
    # Configure storage using the Volume Group
    add_storage: Optional[bool] = Field(default=None)
    # The block device you want to create the volume group on
    device: str
    # The storage identifier.
    name: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseChildrenItemChildrenItem(BaseModel):
    # The free bytes in the physical volume
    free: int
    leaf: bool
    # The name of the physical volume
    name: str
    # The size of the physical volume in bytes
    size: int

    class Config(CommonPydanticConfig):
        pass


class GetResponseChildrenItem(BaseModel):
    # The underlying physical volumes
    children: Optional[list[GetResponseChildrenItemChildrenItem]] = Field(default=None)
    # The free bytes in the volume group
    free: int
    leaf: bool
    # The name of the volume group
    name: str
    # The size of the volume group in bytes
    size: int

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    children: list[GetResponseChildrenItem]
    leaf: bool

    class Config(CommonPydanticConfig):
        pass


@dataclass
class LvmClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'lvm'}"

    def name(self, name: str) -> _name.NameClient:
        return _name.NameClient(
            self.client,
            self.path,
            name,
        )

    def get(self) -> GetResponseItem:
        """
        List LVM Volume Groups
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def post(self, parameters: PostParameters) -> str:
        """
        Create an LVM Volume Group
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncLvmClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'lvm'}"

    def name(self, name: str) -> _name.AsyncNameClient:
        return _name.AsyncNameClient(
            self.client,
            self.path,
            name,
        )

    async def get(self) -> GetResponseItem:
        """
        List LVM Volume Groups
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def post(self, parameters: PostParameters) -> str:
        """
        Create an LVM Volume Group
        """
        return await self.client.post(self.path, parameters, parse_as=str)
