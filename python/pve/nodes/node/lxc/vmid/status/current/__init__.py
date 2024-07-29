from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class Ha(BaseModel):
    """
    HA manager service status.
    """

    pass


class GetResponseItem(BaseModel):
    # Maximum usable CPUs.
    cpus: Optional[float] = Field(default=None)
    # HA manager service status.
    ha: Ha
    # The current config lock, if any.
    lock: Optional[str] = Field(default=None)
    # Root disk size in bytes.
    maxdisk: Optional[int] = Field(default=None)
    # Maximum memory in bytes.
    maxmem: Optional[int] = Field(default=None)
    # Maximum SWAP memory in bytes.
    maxswap: Optional[int] = Field(default=None)
    # Container name.
    name: Optional[str] = Field(default=None)
    # LXC Container status.
    status: str
    # The current configured tags, if any.
    tags: Optional[str] = Field(default=None)
    # Uptime.
    uptime: Optional[int] = Field(default=None)
    # The (unique) ID of the VM.
    vmid: int

    class Config(CommonPydanticConfig):
        pass


@dataclass
class CurrentClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'current'}"

    def get(self) -> GetResponseItem:
        """
        Get virtual machine status.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)


@dataclass
class AsyncCurrentClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'current'}"

    async def get(self) -> GetResponseItem:
        """
        Get virtual machine status.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)
