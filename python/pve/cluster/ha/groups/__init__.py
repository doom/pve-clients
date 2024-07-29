from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import group as _group


class PostParameters(BaseModel):
    # Description.
    comment: Optional[str] = Field(default=None)
    # The HA group identifier.
    group: str
    # List of cluster node names with optional priority.
    nodes: str
    # The CRM tries to run services on the node with the highest priority. If a node with higher priority comes online, the CRM migrates the service to that node. Enabling nofailback prevents that behavior.
    nofailback: Optional[bool] = Field(default=None)
    # Resources bound to restricted groups may only run on nodes defined by the group.
    restricted: Optional[bool] = Field(default=None)
    # Group type.
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    group: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class GroupsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'groups'}"

    def group(self, group: str) -> _group.GroupClient:
        return _group.GroupClient(
            self.client,
            self.path,
            group,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Get HA groups.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create a new HA group.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncGroupsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'groups'}"

    def group(self, group: str) -> _group.AsyncGroupClient:
        return _group.AsyncGroupClient(
            self.client,
            self.path,
            group,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Get HA groups.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters):
        """
        Create a new HA group.
        """
        return await self.client.post(self.path, parameters)
