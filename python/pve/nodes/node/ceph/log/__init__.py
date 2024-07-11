from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    # Line number
    n: int
    # Line text
    t: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    limit: Optional[int] = Field(default=None)
    start: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class LogClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'log'}"

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Read ceph log
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncLogClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'log'}"

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Read ceph log
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
