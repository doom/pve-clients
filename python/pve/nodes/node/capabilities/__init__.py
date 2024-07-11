from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import qemu as _qemu


class GetResponseItem(BaseModel):
    pass


@dataclass
class CapabilitiesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'capabilities'}"

    def qemu(self) -> _qemu.QemuClient:
        return _qemu.QemuClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Node capabilities index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncCapabilitiesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'capabilities'}"

    def qemu(self) -> _qemu.AsyncQemuClient:
        return _qemu.AsyncQemuClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Node capabilities index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
