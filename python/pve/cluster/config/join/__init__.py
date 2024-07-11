from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # Certificate SHA 256 fingerprint.
    fingerprint: str
    # Do not throw error if node already exists.
    force: Optional[bool] = Field(default=None)
    # Hostname (or IP) of an existing cluster member.
    hostname: str
    # Address and priority information of a single corosync link. (up to 8 links supported; link0..link7)
    links: dict[int, Optional[str]] = Field(alias="link[n]", default=None)
    # Node id for this node.
    nodeid: Optional[int] = Field(default=None)
    # Superuser (root) password of peer node.
    password: str
    # Number of votes for this node
    votes: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class Totem(BaseModel):
    pass


class GetResponseNodelistItem(BaseModel):
    # The cluster node name.
    name: str
    # Node id for this node.
    nodeid: Optional[int] = Field(default=None)
    pve_addr: str
    # Certificate SHA 256 fingerprint.
    pve_fp: str
    quorum_votes: int
    # Address and priority information of a single corosync link. (up to 8 links supported; link0..link7)
    ring0_addr: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    config_digest: str
    nodelist: list[GetResponseNodelistItem]
    # The cluster node name.
    preferred_node: str
    totem: Totem

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # The node for which the joinee gets the nodeinfo.
    node: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class JoinClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'join'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get information needed to join this cluster over the connected node.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)

    def post(self, parameters: PostParameters) -> str:
        """
        Joins this node into an existing cluster. If no links are given, default to IP resolved by node's hostname on single link (fallback fails for clusters with multiple links).
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncJoinClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'join'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get information needed to join this cluster over the connected node.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)

    async def post(self, parameters: PostParameters) -> str:
        """
        Joins this node into an existing cluster. If no links are given, default to IP resolved by node's hostname on single link (fallback fails for clusters with multiple links).
        """
        return await self.client.post(self.path, parameters, parse_as=str)
