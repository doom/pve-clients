from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class DeleteParameters(BaseModel):
    # Marks associated storage(s) as not available on this node anymore or removes them from the configuration (if configured for this node only).
    cleanup_config: Optional[bool] = Field(alias="cleanup-config", default=None)
    # Also wipe disk so it can be repurposed afterwards.
    cleanup_disks: Optional[bool] = Field(alias="cleanup-disks", default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class NameClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    def delete(self, parameters: DeleteParameters) -> str:
        """
        Unmounts the storage and removes the mount unit.
        """
        return self.client.delete(self.path, parameters, parse_as=str)


@dataclass
class AsyncNameClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    async def delete(self, parameters: DeleteParameters) -> str:
        """
        Unmounts the storage and removes the mount unit.
        """
        return await self.client.delete(self.path, parameters, parse_as=str)
