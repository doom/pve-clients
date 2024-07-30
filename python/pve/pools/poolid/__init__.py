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
    # Allow adding a guest even if already in another pool. The guest will be removed from its current pool and added to this one.
    allow_move: Optional[bool] = Field(alias="allow-move", default=None)
    comment: Optional[str] = Field(default=None)
    # Remove the passed VMIDs and/or storage IDs instead of adding them.
    delete: Optional[bool] = Field(default=None)
    # List of storage IDs to add or remove from this pool.
    storage: Optional[str] = Field(default=None)
    # List of guest VMIDs to add or remove from this pool.
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
        Delete pool (deprecated, no support for nested pools, use 'DELETE /pools/?poolid={poolid}').
        """
        return self.client.delete(
            self.path,
        )

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get pool configuration (deprecated, no support for nested pools, use 'GET /pools/?poolid={poolid}').
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update pool data (deprecated, no support for nested pools - use 'PUT /pools/?poolid={poolid}' instead).
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
        Delete pool (deprecated, no support for nested pools, use 'DELETE /pools/?poolid={poolid}').
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get pool configuration (deprecated, no support for nested pools, use 'GET /pools/?poolid={poolid}').
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update pool data (deprecated, no support for nested pools - use 'PUT /pools/?poolid={poolid}' instead).
        """
        return await self.client.put(self.path, parameters)
