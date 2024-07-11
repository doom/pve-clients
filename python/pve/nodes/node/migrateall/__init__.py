from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # Maximal number of parallel migration job. If not set, uses'max_workers' from datacenter.cfg. One of both must be set!
    maxworkers: Optional[int] = Field(default=None)
    # Target node.
    target: str
    # Only consider Guests with these IDs.
    vms: Optional[str] = Field(default=None)
    # Enable live storage migration for local disk
    with_local_disks: Optional[bool] = Field(alias="with-local-disks", default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MigrateallClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'migrateall'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Migrate all VMs and Containers.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncMigrateallClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'migrateall'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Migrate all VMs and Containers.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
