from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    # The number of still available instances of this type.
    available: int
    description: str
    # The name of the mdev type.
    type: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MdevClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'mdev'}"

    def get(self) -> list[GetResponseItem]:
        """
        List mediated device types for given PCI device.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncMdevClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'mdev'}"

    async def get(self) -> list[GetResponseItem]:
        """
        List mediated device types for given PCI device.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
