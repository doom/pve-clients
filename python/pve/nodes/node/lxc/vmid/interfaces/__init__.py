from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetResponseItem(BaseModel):
    # The MAC address of the interface
    hwaddr: str
    # The IPv4 address of the interface
    inet: Optional[str] = Field(default=None)
    # The IPv6 address of the interface
    inet6: Optional[str] = Field(default=None)
    # The name of the interface
    name: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class InterfacesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'interfaces'}"

    def get(self) -> list[GetResponseItem]:
        """
        Get IP addresses of the specified container interface.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncInterfacesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'interfaces'}"

    async def get(self) -> list[GetResponseItem]:
        """
        Get IP addresses of the specified container interface.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
