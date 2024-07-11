from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    # Full name of machine type and version.
    id: str
    # The machine type.
    type: str
    # The machine version.
    version: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MachinesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'machines'}"

    def get(self) -> list[GetResponseItem]:
        """
        Get available QEMU/KVM machine types.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncMachinesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'machines'}"

    async def get(self) -> list[GetResponseItem]:
        """
        Get available QEMU/KVM machine types.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
