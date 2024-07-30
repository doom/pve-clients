from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PostParameters(BaseModel):
    # Specify the command.
    command: str

    class Config(CommonPydanticConfig):
        pass


class CurrentKernel(BaseModel):
    """
    The uptime of the system in seconds.
    """

    # Hardware (architecture) type
    machine: str
    # OS kernel release (e.g., \"6.8.0\")
    release: str
    # OS kernel name (e.g., \"Linux\")
    sysname: str
    # OS kernel version with build info
    version: str

    class Config(CommonPydanticConfig):
        pass


class BootInfo(BaseModel):
    """
    Meta-information about the boot mode.
    """

    # Through which firmware the system got booted.
    mode: str
    # System is booted in secure mode, only applicable for the \"efi\" mode.
    secureboot: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Meta-information about the boot mode.
    boot_info: BootInfo = Field(alias="boot-info")
    # The uptime of the system in seconds.
    current_kernel: CurrentKernel = Field(alias="current-kernel")

    class Config(CommonPydanticConfig):
        pass


@dataclass
class StatusClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'status'}"

    def get(self) -> GetResponseItem:
        """
        Read node status
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def post(self, parameters: PostParameters):
        """
        Reboot or shutdown a node.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncStatusClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'status'}"

    async def get(self) -> GetResponseItem:
        """
        Read node status
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def post(self, parameters: PostParameters):
        """
        Reboot or shutdown a node.
        """
        return await self.client.post(self.path, parameters)
