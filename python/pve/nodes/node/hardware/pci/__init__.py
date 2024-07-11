from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import pciid as _pciid


class GetResponseItem(BaseModel):
    # The PCI Class of the device.
    class_: str = Field(alias="class")
    # The Device ID.
    device: str
    device_name: Optional[str] = Field(default=None)
    # The PCI ID.
    id: str
    # The IOMMU group in which the device is in. If no IOMMU group is detected, it is set to -1.
    iommugroup: int
    # If set, marks that the device is capable of creating mediated devices.
    mdev: Optional[bool] = Field(default=None)
    # The Subsystem Device ID.
    subsystem_device: Optional[str] = Field(default=None)
    subsystem_device_name: Optional[str] = Field(default=None)
    # The Subsystem Vendor ID.
    subsystem_vendor: Optional[str] = Field(default=None)
    subsystem_vendor_name: Optional[str] = Field(default=None)
    # The Vendor ID.
    vendor: str
    vendor_name: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # A list of blacklisted PCI classes, which will not be returned. Following are filtered by default: Memory Controller (05), Bridge (06) and Processor (0b).
    pci_class_blacklist: Optional[str] = Field(
        alias="pci-class-blacklist", default=None
    )
    # If disabled, does only print the PCI IDs. Otherwise, additional information like vendor and device will be returned.
    verbose: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class PciClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'pci'}"

    def pciid(self, pciid: str) -> _pciid.PciidClient:
        return _pciid.PciidClient(
            self.client,
            self.path,
            pciid,
        )

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List local PCI devices.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncPciClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'pci'}"

    def pciid(self, pciid: str) -> _pciid.AsyncPciidClient:
        return _pciid.AsyncPciidClient(
            self.client,
            self.path,
            pciid,
        )

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List local PCI devices.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
