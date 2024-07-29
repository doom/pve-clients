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
    node: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class NodesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'nodes'}"

    def node(self, node: str) -> _node.NodeClient:
        return _node.NodeClient(
            self.client,
            self.path,
            node,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Corosync node list.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncNodesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'nodes'}"

    def node(self, node: str) -> _node.AsyncNodeClient:
        return _node.AsyncNodeClient(
            self.client,
            self.path,
            node,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Corosync node list.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
