from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import subnet as _subnet


class PostParameters(BaseModel):
    # IP address for the DNS server
    dhcp_dns_server: Optional[str] = Field(alias="dhcp-dns-server", default=None)
    # A list of DHCP ranges for this subnet
    dhcp_range: Optional[list[str]] = Field(alias="dhcp-range", default=None)
    # dns domain zone prefix  ex: 'adm' -> <hostname>.adm.mydomain.com
    dnszoneprefix: Optional[str] = Field(default=None)
    # Subnet Gateway: Will be assign on vnet for layer3 zones
    gateway: Optional[str] = Field(default=None)
    # enable masquerade for this subnet if pve-firewall
    snat: Optional[bool] = Field(default=None)
    # The SDN subnet object identifier.
    subnet: str
    type: str

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
class SubnetsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'subnets'}"

    def subnet(self, subnet: str) -> _subnet.SubnetClient:
        return _subnet.SubnetClient(
            self.client,
            self.path,
            subnet,
        )

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        SDN subnets index.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create a new sdn subnet object.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncSubnetsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'subnets'}"

    def subnet(self, subnet: str) -> _subnet.AsyncSubnetClient:
        return _subnet.AsyncSubnetClient(
            self.client,
            self.path,
            subnet,
        )

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        SDN subnets index.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )

    async def post(self, parameters: PostParameters):
        """
        Create a new sdn subnet object.
        """
        return await self.client.post(self.path, parameters)
