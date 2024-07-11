from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import apiversion as _apiversion
from . import join as _join
from . import nodes as _nodes
from . import qdevice as _qdevice
from . import totem as _totem


class PostParameters(BaseModel):
    # The name of the cluster.
    clustername: str
    # Address and priority information of a single corosync link. (up to 8 links supported; link0..link7)
    links: dict[int, Optional[str]] = Field(alias="link[n]", default=None)
    # Node id for this node.
    nodeid: Optional[int] = Field(default=None)
    # Number of votes for this node.
    votes: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


@dataclass
class ConfigClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'config'}"

    def apiversion(self) -> _apiversion.ApiversionClient:
        return _apiversion.ApiversionClient(
            self.client,
            self.path,
        )

    def nodes(self) -> _nodes.NodesClient:
        return _nodes.NodesClient(
            self.client,
            self.path,
        )

    def join(self) -> _join.JoinClient:
        return _join.JoinClient(
            self.client,
            self.path,
        )

    def totem(self) -> _totem.TotemClient:
        return _totem.TotemClient(
            self.client,
            self.path,
        )

    def qdevice(self) -> _qdevice.QdeviceClient:
        return _qdevice.QdeviceClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters) -> str:
        """
        Generate new cluster configuration. If no links given, default to local IP address as link0.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncConfigClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'config'}"

    def apiversion(self) -> _apiversion.AsyncApiversionClient:
        return _apiversion.AsyncApiversionClient(
            self.client,
            self.path,
        )

    def nodes(self) -> _nodes.AsyncNodesClient:
        return _nodes.AsyncNodesClient(
            self.client,
            self.path,
        )

    def join(self) -> _join.AsyncJoinClient:
        return _join.AsyncJoinClient(
            self.client,
            self.path,
        )

    def totem(self) -> _totem.AsyncTotemClient:
        return _totem.AsyncTotemClient(
            self.client,
            self.path,
        )

    def qdevice(self) -> _qdevice.AsyncQdeviceClient:
        return _qdevice.AsyncQdeviceClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters) -> str:
        """
        Generate new cluster configuration. If no links given, default to local IP address as link0.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
