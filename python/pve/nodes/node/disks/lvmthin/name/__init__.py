from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class DeleteParameters(BaseModel):
    # Marks associated storage(s) as not available on this node anymore or removes them from the configuration (if configured for this node only).
    cleanup_config: Optional[bool] = Field(alias="cleanup-config", default=None)
    # Also wipe disks so they can be repurposed afterwards.
    cleanup_disks: Optional[bool] = Field(alias="cleanup-disks", default=None)
    # The storage identifier.
    volume_group: str = Field(alias="volume-group")

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
        Remove an LVM thin pool.
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
        Remove an LVM thin pool.
        """
        return await self.client.delete(self.path, parameters, parse_as=str)
