from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PutParameters(BaseModel):
    # Description.
    comment: Optional[str] = Field(default=None)
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Flag to disable/deactivate the entry.
    disable: Optional[bool] = Field(default=None)
    # Rate limit in mbps (megabytes per second) as floating point number.
    rate: Optional[float] = Field(default=None)
    # Mark the replication job for removal. The job will remove all local replication snapshots. When set to 'full', it also tries to remove replicated volumes on the target. The job then removes itself from the configuration file.
    remove_job: Optional[str] = Field(default=None)
    # Storage replication schedule. The format is a subset of `systemd` calendar events.
    schedule: Optional[str] = Field(default=None)
    # For internal use, to detect if the guest was stolen.
    source: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


class DeleteParameters(BaseModel):
    # Will remove the jobconfig entry, but will not cleanup.
    force: Optional[bool] = Field(default=None)
    # Keep replicated data at target (do not remove).
    keep: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class IdClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, id: str):
        self.client = client
        self.path = f"{parent_path}/{id}"

    def delete(self, parameters: DeleteParameters):
        """
        Mark replication job for removal.
        """
        return self.client.delete(self.path, parameters)

    def get(self) -> GetResponseItem:
        """
        Read replication job configuration.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update replication job configuration.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncIdClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, id: str):
        self.client = client
        self.path = f"{parent_path}/{id}"

    async def delete(self, parameters: DeleteParameters):
        """
        Mark replication job for removal.
        """
        return await self.client.delete(self.path, parameters)

    async def get(self) -> GetResponseItem:
        """
        Read replication job configuration.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update replication job configuration.
        """
        return await self.client.put(self.path, parameters)
