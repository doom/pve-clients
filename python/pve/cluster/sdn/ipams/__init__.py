from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import ipam as _ipam


class PostParameters(BaseModel):
    # The SDN ipam object identifier.
    ipam: str
    section: Optional[int] = Field(default=None)
    token: Optional[str] = Field(default=None)
    # Plugin type.
    type: str
    url: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    ipam: str
    type: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Only list sdn ipams of specific type
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class IpamsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'ipams'}"

    def ipam(self, ipam: str) -> _ipam.IpamClient:
        return _ipam.IpamClient(
            self.client,
            self.path,
            ipam,
        )

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        SDN ipams index.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create a new sdn ipam object.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncIpamsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'ipams'}"

    def ipam(self, ipam: str) -> _ipam.AsyncIpamClient:
        return _ipam.AsyncIpamClient(
            self.client,
            self.path,
            ipam,
        )

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        SDN ipams index.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )

    async def post(self, parameters: PostParameters):
        """
        Create a new sdn ipam object.
        """
        return await self.client.post(self.path, parameters)
