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
    # The IP address to associate with the given MAC address
    ip: str
    # Unicast MAC address.
    mac: Optional[str] = Field(default=None)
    # The (unique) ID of the VM.
    vmid: Optional[int] = Field(default=None)
    # The SDN zone object identifier.
    zone: str

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # The IP address to associate with the given MAC address
    ip: str
    # Unicast MAC address.
    mac: Optional[str] = Field(default=None)
    # The SDN zone object identifier.
    zone: str

    class Config(CommonPydanticConfig):
        pass


class DeleteParameters(BaseModel):
    # The IP address to delete
    ip: str
    # Unicast MAC address.
    mac: Optional[str] = Field(default=None)
    # The SDN zone object identifier.
    zone: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class IpsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'ips'}"

    def delete(self, parameters: DeleteParameters):
        """
        Delete IP Mappings in a VNet
        """
        return self.client.delete(self.path, parameters)

    def post(self, parameters: PostParameters):
        """
        Create IP Mapping in a VNet
        """
        return self.client.post(self.path, parameters)

    def put(self, parameters: PutParameters):
        """
        Update IP Mapping in a VNet
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncIpsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'ips'}"

    async def delete(self, parameters: DeleteParameters):
        """
        Delete IP Mappings in a VNet
        """
        return await self.client.delete(self.path, parameters)

    async def post(self, parameters: PostParameters):
        """
        Create IP Mapping in a VNet
        """
        return await self.client.post(self.path, parameters)

    async def put(self, parameters: PutParameters):
        """
        Update IP Mapping in a VNet
        """
        return await self.client.put(self.path, parameters)
