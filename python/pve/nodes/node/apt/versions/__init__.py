from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    pass


@dataclass
class VersionsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'versions'}"

    def get(self) -> list[GetResponseItem]:
        """
        Get package information for important Proxmox packages.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncVersionsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'versions'}"

    async def get(self) -> list[GetResponseItem]:
        """
        Get package information for important Proxmox packages.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
