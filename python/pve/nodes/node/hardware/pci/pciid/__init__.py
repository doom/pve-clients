from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import mdev as _mdev


class GetResponseItem(BaseModel):
    method: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class PciidClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, pciid: str):
        self.client = client
        self.path = f"{parent_path}/{pciid}"

    def mdev(self) -> _mdev.MdevClient:
        return _mdev.MdevClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Index of available pci methods
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncPciidClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, pciid: str):
        self.client = client
        self.path = f"{parent_path}/{pciid}"

    def mdev(self) -> _mdev.AsyncMdevClient:
        return _mdev.AsyncMdevClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Index of available pci methods
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
