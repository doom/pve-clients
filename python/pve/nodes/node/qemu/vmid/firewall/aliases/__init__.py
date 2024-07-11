from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import name as _name


class PostParameters(BaseModel):
    # Network/IP specification in CIDR format.
    cidr: str
    comment: Optional[str] = Field(default=None)
    # Alias name.
    name: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    cidr: str
    comment: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: str
    name: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class AliasesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'aliases'}"

    def name(self, name: str) -> _name.NameClient:
        return _name.NameClient(
            self.client,
            self.path,
            name,
        )

    def get(self) -> list[GetResponseItem]:
        """
        List aliases
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create IP or Network Alias.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncAliasesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'aliases'}"

    def name(self, name: str) -> _name.AsyncNameClient:
        return _name.AsyncNameClient(
            self.client,
            self.path,
            name,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        List aliases
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters):
        """
        Create IP or Network Alias.
        """
        return await self.client.post(self.path, parameters)
