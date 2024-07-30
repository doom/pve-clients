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
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # IP address for the DNS server
    dhcp_dns_server: Optional[str] = Field(alias="dhcp-dns-server", default=None)
    # A list of DHCP ranges for this subnet
    dhcp_range: Optional[list[str]] = Field(alias="dhcp-range", default=None)
    # Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # dns domain zone prefix  ex: 'adm' -> <hostname>.adm.mydomain.com
    dnszoneprefix: Optional[str] = Field(default=None)
    # Subnet Gateway: Will be assign on vnet for layer3 zones
    gateway: Optional[str] = Field(default=None)
    # enable masquerade for this subnet if pve-firewall
    snat: Optional[bool] = Field(default=None)

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
class SubnetClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, subnet: str):
        self.client = client
        self.path = f"{parent_path}/{subnet}"

    def delete(self):
        """
        Delete sdn subnet object configuration.
        """
        return self.client.delete(
            self.path,
        )

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Read sdn subnet configuration.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update sdn subnet object configuration.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncSubnetClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, subnet: str):
        self.client = client
        self.path = f"{parent_path}/{subnet}"

    async def delete(self):
        """
        Delete sdn subnet object configuration.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Read sdn subnet configuration.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update sdn subnet object configuration.
        """
        return await self.client.put(self.path, parameters)
