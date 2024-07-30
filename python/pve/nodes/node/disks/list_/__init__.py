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
    # The device path
    devpath: str
    gpt: bool
    health: Optional[str] = Field(default=None)
    model: Optional[str] = Field(default=None)
    mounted: bool
    osdid: int
    osdid_list: list[int] = Field(alias="osdid-list")
    # For partitions only. The device path of the disk the partition resides on.
    parent: Optional[str] = Field(default=None)
    serial: Optional[str] = Field(default=None)
    size: int
    used: Optional[str] = Field(default=None)
    vendor: Optional[str] = Field(default=None)
    wwn: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Also include partitions.
    include_partitions: Optional[bool] = Field(alias="include-partitions", default=None)
    # Skip smart checks.
    skipsmart: Optional[bool] = Field(default=None)
    # Only list specific types of disks.
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ListClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'list'}"

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List local disks.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncListClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'list'}"

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List local disks.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
