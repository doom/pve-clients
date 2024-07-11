from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import poolid as _poolid


class PostParameters(BaseModel):
    comment: Optional[str] = Field(default=None)
    poolid: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
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

    def get(self) -> list[GetResponseItem]:
        """
        Pool index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create new pool.
        """
        return self.client.post(self.path, parameters)


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

    async def get(self) -> list[GetResponseItem]:
        """
        Pool index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters):
        """
        Create new pool.
        """
        return await self.client.post(self.path, parameters)
