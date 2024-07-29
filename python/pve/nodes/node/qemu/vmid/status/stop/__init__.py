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
    # Do not deactivate storage volumes.
    keep_active: Optional[bool] = Field(alias="keepActive", default=None)
    # The cluster node name.
    migratedfrom: Optional[str] = Field(default=None)
    # Ignore locks - only root is allowed to use this option.
    skiplock: Optional[bool] = Field(default=None)
    # Wait maximal timeout seconds.
    timeout: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class StopClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'stop'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Stop virtual machine. The qemu process will exit immediately. Thisis akin to pulling the power plug of a running computer and may damage the VM data
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncStopClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'stop'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Stop virtual machine. The qemu process will exit immediately. Thisis akin to pulling the power plug of a running computer and may damage the VM data
        """
        return await self.client.post(self.path, parameters, parse_as=str)
