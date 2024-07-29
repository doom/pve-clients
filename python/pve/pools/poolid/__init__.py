from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PutParameters(BaseModel):
    comment: Optional[str] = Field(default=None)
    # Remove vms/storage (instead of adding it).
    delete: Optional[bool] = Field(default=None)
    # List of storage IDs.
    storage: Optional[str] = Field(default=None)
    # List of virtual machines.
    vms: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseMembersItem(BaseModel):
    id: str
    node: str
    storage: Optional[str] = Field(default=None)
    type: str
    vmid: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    comment: Optional[str] = Field(default=None)
    members: list[GetResponseMembersItem]

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class PoolidClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, poolid: str):
        self.client = client
        self.path = f"{parent_path}/{poolid}"

    def delete(self):
        """
        Delete pool.
        """
        return self.client.delete(
            self.path,
        )

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get pool configuration.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update pool data.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncPoolidClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, poolid: str):
        self.client = client
        self.path = f"{parent_path}/{poolid}"

    async def delete(self):
        """
        Delete pool.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get pool configuration.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update pool data.
        """
        return await self.client.put(self.path, parameters)
