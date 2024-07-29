from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import pci as _pci
from . import usb as _usb


class GetResponseItem(BaseModel):
    type: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class HardwareClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'hardware'}"

    def pci(self) -> _pci.PciClient:
        return _pci.PciClient(
            self.client,
            self.path,
        )

    def usb(self) -> _usb.UsbClient:
        return _usb.UsbClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Index of hardware types
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncHardwareClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'hardware'}"

    def pci(self) -> _pci.AsyncPciClient:
        return _pci.AsyncPciClient(
            self.client,
            self.path,
        )

    def usb(self) -> _usb.AsyncUsbClient:
        return _usb.AsyncUsbClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Index of hardware types
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
