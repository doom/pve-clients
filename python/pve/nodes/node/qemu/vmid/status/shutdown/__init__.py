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
    # Make sure the VM stops.
    force_stop: Optional[bool] = Field(alias="forceStop", default=None)
    # Do not deactivate storage volumes.
    keep_active: Optional[bool] = Field(alias="keepActive", default=None)
    # Ignore locks - only root is allowed to use this option.
    skiplock: Optional[bool] = Field(default=None)
    # Wait maximal timeout seconds.
    timeout: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ShutdownClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'shutdown'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Shutdown virtual machine. This is similar to pressing the power button on a physical machine. This will send an ACPI event for the guest OS, which should then proceed to a clean shutdown.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncShutdownClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'shutdown'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Shutdown virtual machine. This is similar to pressing the power button on a physical machine. This will send an ACPI event for the guest OS, which should then proceed to a clean shutdown.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
