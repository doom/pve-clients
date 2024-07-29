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
    # First name server IP address.
    dns1: Optional[str] = Field(default=None)
    # Second name server IP address.
    dns2: Optional[str] = Field(default=None)
    # Third name server IP address.
    dns3: Optional[str] = Field(default=None)
    # Search domain for host-name lookup.
    search: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # First name server IP address.
    dns1: Optional[str] = Field(default=None)
    # Second name server IP address.
    dns2: Optional[str] = Field(default=None)
    # Third name server IP address.
    dns3: Optional[str] = Field(default=None)
    # Search domain for host-name lookup.
    search: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class DnsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'dns'}"

    def get(self) -> GetResponseItem:
        """
        Read DNS settings.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Write DNS settings.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncDnsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'dns'}"

    async def get(self) -> GetResponseItem:
        """
        Read DNS settings.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Write DNS settings.
        """
        return await self.client.put(self.path, parameters)
