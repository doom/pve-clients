from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import id as _id


class PostParameters(BaseModel):
    # Description.
    comment: Optional[str] = Field(default=None)
    # Flag to disable/deactivate the entry.
    disable: Optional[bool] = Field(default=None)
    # Replication Job ID. The ID is composed of a Guest ID and a job number, separated by a hyphen, i.e. '<GUEST>-<JOBNUM>'.
    id: str
    # Rate limit in mbps (megabytes per second) as floating point number.
    rate: Optional[float] = Field(default=None)
    # Mark the replication job for removal. The job will remove all local replication snapshots. When set to 'full', it also tries to remove replicated volumes on the target. The job then removes itself from the configuration file.
    remove_job: Optional[str] = Field(default=None)
    # Storage replication schedule. The format is a subset of `systemd` calendar events.
    schedule: Optional[str] = Field(default=None)
    # For internal use, to detect if the guest was stolen.
    source: Optional[str] = Field(default=None)
    # Target node.
    target: str
    # Section type.
    type: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


@dataclass
class ReplicationClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'replication'}"

    def id(self, id: str) -> _id.IdClient:
        return _id.IdClient(
            self.client,
            self.path,
            id,
        )

    def get(self) -> list[GetResponseItem]:
        """
        List replication jobs.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create a new replication job
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncReplicationClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'replication'}"

    def id(self, id: str) -> _id.AsyncIdClient:
        return _id.AsyncIdClient(
            self.client,
            self.path,
            id,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        List replication jobs.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters):
        """
        Create a new replication job
        """
        return await self.client.post(self.path, parameters)
