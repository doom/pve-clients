from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import ips as _ips
from . import subnets as _subnets


class PutParameters(BaseModel):
    # alias name of the vnet
    alias: Optional[str] = Field(default=None)
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # vlan or vxlan id
    tag: Optional[int] = Field(default=None)
    # Allow vm VLANs to pass through this vnet.
    vlanaware: Optional[bool] = Field(default=None)
    # zone id
    zone: Optional[str] = Field(default=None)

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
class VnetClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, vnet: str):
        self.client = client
        self.path = f"{parent_path}/{vnet}"

    def subnets(self) -> _subnets.SubnetsClient:
        return _subnets.SubnetsClient(
            self.client,
            self.path,
        )

    def ips(self) -> _ips.IpsClient:
        return _ips.IpsClient(
            self.client,
            self.path,
        )

    def delete(self):
        """
        Delete sdn vnet object configuration.
        """
        return self.client.delete(
            self.path,
        )

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Read sdn vnet configuration.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update sdn vnet object configuration.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncVnetClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, vnet: str):
        self.client = client
        self.path = f"{parent_path}/{vnet}"

    def subnets(self) -> _subnets.AsyncSubnetsClient:
        return _subnets.AsyncSubnetsClient(
            self.client,
            self.path,
        )

    def ips(self) -> _ips.AsyncIpsClient:
        return _ips.AsyncIpsClient(
            self.client,
            self.path,
        )

    async def delete(self):
        """
        Delete sdn vnet object configuration.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Read sdn vnet configuration.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update sdn vnet object configuration.
        """
        return await self.client.put(self.path, parameters)
