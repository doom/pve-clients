from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import node as _node


class GetResponseItem(BaseModel):
    # CPU utilization.
    cpu: Optional[float] = Field(default=None)
    # Support level.
    level: Optional[str] = Field(default=None)
    # Number of available CPUs.
    maxcpu: Optional[int] = Field(default=None)
    # Number of available memory in bytes.
    maxmem: Optional[int] = Field(default=None)
    # Used memory in bytes.
    mem: Optional[int] = Field(default=None)
    # The cluster node name.
    node: str
    # The SSL fingerprint for the node certificate.
    ssl_fingerprint: Optional[str] = Field(default=None)
    # Node status.
    status: str
    # Node uptime in seconds.
    uptime: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class NodesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient):
        self.client = client
        self.path = "nodes"

    def node(self, node: str) -> _node.NodeClient:
        return _node.NodeClient(
            self.client,
            self.path,
            node,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Cluster node index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncNodesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient):
        self.client = client
        self.path = "nodes"

    def node(self, node: str) -> _node.AsyncNodeClient:
        return _node.AsyncNodeClient(
            self.client,
            self.path,
            node,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Cluster node index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
