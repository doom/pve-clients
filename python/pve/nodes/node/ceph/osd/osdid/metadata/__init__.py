from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class Osd(BaseModel):
    """
    General information about the OSD
    """

    # Address and port used to talk to other OSDs.
    back_addr: str
    # Address and port used to talk to clients and monitors.
    front_addr: str
    # Heartbeat address and port for other OSDs.
    hb_back_addr: str
    # Heartbeat address and port for clients and monitors.
    hb_front_addr: str
    # Name of the host containing the OSD.
    hostname: str
    # ID of the OSD.
    id: int
    # Memory usage of the OSD service.
    mem_usage: int
    # Path to the OSD's data directory.
    osd_data: str
    # The type of object store used.
    osd_objectstore: str
    # OSD process ID.
    pid: int
    # Ceph version of the OSD service.
    version: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseDevicesItem(BaseModel):
    # Device node
    dev_node: str
    # Kind of OSD device
    device: str
    # Physical disks used
    devices: str
    # Size in bytes
    size: int
    # Discard support of the physical device
    support_discard: bool
    # Type of device. For example, hdd or ssd
    type: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Array containing data about devices
    devices: list[GetResponseDevicesItem]
    # General information about the OSD
    osd: Osd

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MetadataClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'metadata'}"

    def get(self) -> GetResponseItem:
        """
        Get OSD details
        """
        return self.client.get(self.path, parse_as=GetResponseItem)


@dataclass
class AsyncMetadataClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'metadata'}"

    async def get(self) -> GetResponseItem:
        """
        Get OSD details
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)
