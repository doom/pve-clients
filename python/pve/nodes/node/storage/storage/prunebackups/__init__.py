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
    # Creation time of the backup (seconds since the UNIX epoch).
    ctime: int
    # Whether the backup would be kept or removed. Backups that are protected or don't use the standard naming scheme are not removed.
    mark: str
    # One of 'qemu', 'lxc', 'openvz' or 'unknown'.
    type: str
    # The VM the backup belongs to.
    vmid: Optional[int] = Field(default=None)
    # Backup volume ID.
    volid: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Use these retention options instead of those from the storage configuration.
    prune_backups: Optional[str] = Field(alias="prune-backups", default=None)
    # Either 'qemu' or 'lxc'. Only consider backups for guests of this type.
    type: Optional[str] = Field(default=None)
    # Only consider backups for this guest.
    vmid: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class DeleteParameters(BaseModel):
    # Use these retention options instead of those from the storage configuration.
    prune_backups: Optional[str] = Field(alias="prune-backups", default=None)
    # Either 'qemu' or 'lxc'. Only consider backups for guests of this type.
    type: Optional[str] = Field(default=None)
    # Only prune backups for this VM.
    vmid: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class PrunebackupsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'prunebackups'}"

    def delete(self, parameters: DeleteParameters) -> str:
        """
        Prune backups. Only those using the standard naming scheme are considered.
        """
        return self.client.delete(self.path, parameters, parse_as=str)

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Get prune information for backups. NOTE: this is only a preview and might not be what a subsequent prune call does if backups are removed/added in the meantime.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncPrunebackupsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'prunebackups'}"

    async def delete(self, parameters: DeleteParameters) -> str:
        """
        Prune backups. Only those using the standard naming scheme are considered.
        """
        return await self.client.delete(self.path, parameters, parse_as=str)

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Get prune information for backups. NOTE: this is only a preview and might not be what a subsequent prune call does if backups are removed/added in the meantime.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
