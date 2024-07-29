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
    # List of cluster node names with optional priority.
    nodes: Optional[str] = Field(default=None)
    # The CRM tries to run services on the node with the highest priority. If a node with higher priority comes online, the CRM migrates the service to that node. Enabling nofailback prevents that behavior.
    nofailback: Optional[bool] = Field(default=None)
    # Resources bound to restricted groups may only run on nodes defined by the group.
    restricted: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class GroupClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, group: str):
        self.client = client
        self.path = f"{parent_path}/{group}"

    def delete(self):
        """
        Delete ha group configuration.
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> dict[str, Any]:
        """
        Read ha group configuration.
        """
        return self.client.get(self.path, parse_as=dict[str, Any])

    def put(self, parameters: PutParameters):
        """
        Update ha group configuration.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncGroupClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, group: str):
        self.client = client
        self.path = f"{parent_path}/{group}"

    async def delete(self):
        """
        Delete ha group configuration.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> dict[str, Any]:
        """
        Read ha group configuration.
        """
        return await self.client.get(self.path, parse_as=dict[str, Any])

    async def put(self, parameters: PutParameters):
        """
        Update ha group configuration.
        """
        return await self.client.put(self.path, parameters)
