from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import id as _id


class GetResponseItem(BaseModel):
    id: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Only list replication jobs for this guest.
    guest: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
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

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List status of all replication jobs on this node.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


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

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List status of all replication jobs on this node.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
