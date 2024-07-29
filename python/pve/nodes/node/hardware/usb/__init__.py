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
    busnum: int
    class_: int = Field(alias="class")
    devnum: int
    level: int
    manufacturer: Optional[str] = Field(default=None)
    port: int
    prodid: str
    product: Optional[str] = Field(default=None)
    serial: Optional[str] = Field(default=None)
    speed: str
    usbpath: Optional[str] = Field(default=None)
    vendid: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class UsbClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'usb'}"

    def get(self) -> list[GetResponseItem]:
        """
        List local USB devices.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncUsbClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'usb'}"

    async def get(self) -> list[GetResponseItem]:
        """
        List local USB devices.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
