from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import vnet as _vnet


class PostParameters(BaseModel):
    # alias name of the vnet
    alias: Optional[str] = Field(default=None)
    # vlan or vxlan id
    tag: Optional[int] = Field(default=None)
    # Type
    type: Optional[str] = Field(default=None)
    # Allow vm VLANs to pass through this vnet.
    vlanaware: Optional[bool] = Field(default=None)
    # The SDN vnet object identifier.
    vnet: str
    # zone id
    zone: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


class GetParameters(BaseModel):
    # Display pending config.
    pending: Optional[bool] = Field(default=None)
    # Display running config.
    running: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class VnetsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'vnets'}"

    def vnet(self, vnet: str) -> _vnet.VnetClient:
        return _vnet.VnetClient(
            self.client,
            self.path,
            vnet,
        )

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        SDN vnets index.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create a new sdn vnet object.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncVnetsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'vnets'}"

    def vnet(self, vnet: str) -> _vnet.AsyncVnetClient:
        return _vnet.AsyncVnetClient(
            self.client,
            self.path,
            vnet,
        )

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        SDN vnets index.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )

    async def post(self, parameters: PostParameters):
        """
        Create a new sdn vnet object.
        """
        return await self.client.post(self.path, parameters)
