from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PostResponseItem(BaseModel):
    corosync_authkey: str
    corosync_conf: str
    warnings: list[str]

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # The JOIN_API_VERSION of the new node.
    apiversion: Optional[int] = Field(default=None)
    # Do not throw error if node already exists.
    force: Optional[bool] = Field(default=None)
    # Address and priority information of a single corosync link. (up to 8 links supported; link0..link7)
    links: dict[int, Optional[str]] = Field(default=None)
    # IP Address of node to add. Used as fallback if no links are given.
    new_node_ip: Optional[str] = Field(default=None)
    # Node id for this node.
    nodeid: Optional[int] = Field(default=None)
    # Number of votes for this node
    votes: Optional[int] = Field(default=None)

    @model_serializer(mode="wrap")
    def _serialize_repeated(self, serializer):
        data = serializer(self)
        data = serialize_repeated_with_prefix(data, group="links", prefix="link")
        return data

    @model_validator(mode="before")
    @classmethod
    def _extract_repeated(cls, data: Any) -> Any:
        if not isinstance(data, dict):
            return data
        data = data = extract_repeated_with_prefix(data, group="links", prefix="link")
        return data

    class Config(CommonPydanticConfig):
        pass


@dataclass
class NodeClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, node: str):
        self.client = client
        self.path = f"{parent_path}/{node}"

    def delete(self):
        """
        Removes a node from the cluster configuration.
        """
        return self.client.delete(
            self.path,
        )

    def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Adds a node to the cluster configuration. This call is for internal use.
        """
        return self.client.post(self.path, parameters, parse_as=PostResponseItem)


@dataclass
class AsyncNodeClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, node: str):
        self.client = client
        self.path = f"{parent_path}/{node}"

    async def delete(self):
        """
        Removes a node from the cluster configuration.
        """
        return await self.client.delete(
            self.path,
        )

    async def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Adds a node to the cluster configuration. This call is for internal use.
        """
        return await self.client.post(self.path, parameters, parse_as=PostResponseItem)
