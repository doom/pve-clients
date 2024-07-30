from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import osdid as _osdid


class PostParameters(BaseModel):
    # Set the device class of the OSD in crush.
    crush_device_class: Optional[str] = Field(alias="crush-device-class", default=None)
    # Block device name for block.db.
    db_dev: Optional[str] = Field(default=None)
    # Size in GiB for block.db.
    db_dev_size: Optional[float] = Field(default=None)
    # Block device name.
    dev: str
    # Enables encryption of the OSD.
    encrypted: Optional[bool] = Field(default=None)
    # OSD services per physical device. Only useful for fast NVMe devices\" 		    .\" to utilize their performance better.
    osds_per_device: Optional[int] = Field(alias="osds-per-device", default=None)
    # Block device name for block.wal.
    wal_dev: Optional[str] = Field(default=None)
    # Size in GiB for block.wal.
    wal_dev_size: Optional[float] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


@dataclass
class OsdClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'osd'}"

    def osdid(self, osdid: str) -> _osdid.OsdidClient:
        return _osdid.OsdidClient(
            self.client,
            self.path,
            osdid,
        )

    def get(self) -> GetResponseItem:
        """
        Get Ceph osd list/tree.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def post(self, parameters: PostParameters) -> str:
        """
        Create OSD
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncOsdClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'osd'}"

    def osdid(self, osdid: str) -> _osdid.AsyncOsdidClient:
        return _osdid.AsyncOsdidClient(
            self.client,
            self.path,
            osdid,
        )

    async def get(self) -> GetResponseItem:
        """
        Get Ceph osd list/tree.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def post(self, parameters: PostParameters) -> str:
        """
        Create OSD
        """
        return await self.client.post(self.path, parameters, parse_as=str)
