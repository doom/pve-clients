from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import poolid as _poolid


class PutParameters(BaseModel):
    # Allow adding a guest even if already in another pool. The guest will be removed from its current pool and added to this one.
    allow_move: Optional[bool] = Field(alias="allow-move", default=None)
    comment: Optional[str] = Field(default=None)
    # Remove the passed VMIDs and/or storage IDs instead of adding them.
    delete: Optional[bool] = Field(default=None)
    poolid: str
    # List of storage IDs to add or remove from this pool.
    storage: Optional[str] = Field(default=None)
    # List of guest VMIDs to add or remove from this pool.
    vms: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    comment: Optional[str] = Field(default=None)
    poolid: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItemMembersItem(BaseModel):
    id: str
    node: str
    storage: Optional[str] = Field(default=None)
    type: str
    vmid: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    comment: Optional[str] = Field(default=None)
    members: Optional[list[GetResponseItemMembersItem]] = Field(default=None)
    poolid: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    poolid: Optional[str] = Field(default=None)
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class DeleteParameters(BaseModel):
    poolid: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class PoolsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient):
        self.client = client
        self.path = "pools"

    def poolid(self, poolid: str) -> _poolid.PoolidClient:
        return _poolid.PoolidClient(
            self.client,
            self.path,
            poolid,
        )

    def delete(self, parameters: DeleteParameters):
        """
        Delete pool.
        """
        return self.client.delete(self.path, parameters)

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List pools or get pool configuration.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create new pool.
        """
        return self.client.post(self.path, parameters)

    def put(self, parameters: PutParameters):
        """
        Update pool.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncPoolsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient):
        self.client = client
        self.path = "pools"

    def poolid(self, poolid: str) -> _poolid.AsyncPoolidClient:
        return _poolid.AsyncPoolidClient(
            self.client,
            self.path,
            poolid,
        )

    async def delete(self, parameters: DeleteParameters):
        """
        Delete pool.
        """
        return await self.client.delete(self.path, parameters)

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List pools or get pool configuration.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )

    async def post(self, parameters: PostParameters):
        """
        Create new pool.
        """
        return await self.client.post(self.path, parameters)

    async def put(self, parameters: PutParameters):
        """
        Update pool.
        """
        return await self.client.put(self.path, parameters)
