from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import dns as _dns


class PostParameters(BaseModel):
    # The SDN dns object identifier.
    dns: str
    key: str
    reversemaskv6: Optional[int] = Field(default=None)
    reversev6mask: Optional[int] = Field(default=None)
    ttl: Optional[int] = Field(default=None)
    # Plugin type.
    type: str
    url: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    dns: str
    type: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Only list sdn dns of specific type
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class DnsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'dns'}"

    def dns(self, dns: str) -> _dns.DnsClient:
        return _dns.DnsClient(
            self.client,
            self.path,
            dns,
        )

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        SDN dns index.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create a new sdn dns object.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncDnsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'dns'}"

    def dns(self, dns: str) -> _dns.AsyncDnsClient:
        return _dns.AsyncDnsClient(
            self.client,
            self.path,
            dns,
        )

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        SDN dns index.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )

    async def post(self, parameters: PostParameters):
        """
        Create a new sdn dns object.
        """
        return await self.client.post(self.path, parameters)
