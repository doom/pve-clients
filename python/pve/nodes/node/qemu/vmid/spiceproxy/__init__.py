from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostResponseItem(BaseModel):
    """
    Returned values can be directly passed to the 'remote-viewer' application.
    """

    host: str
    password: str
    proxy: str
    tls_port: int = Field(alias="tls-port")
    type: str

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # SPICE proxy server. This can be used by the client to specify the proxy server. All nodes in a cluster runs 'spiceproxy', so it is up to the client to choose one. By default, we return the node where the VM is currently running. As reasonable setting is to use same node you use to connect to the API (This is window.location.hostname for the JS GUI).
    proxy: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class SpiceproxyClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'spiceproxy'}"

    def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Returns a SPICE configuration to connect to the VM.
        """
        return self.client.post(self.path, parameters, parse_as=PostResponseItem)


@dataclass
class AsyncSpiceproxyClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'spiceproxy'}"

    async def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Returns a SPICE configuration to connect to the VM.
        """
        return await self.client.post(self.path, parameters, parse_as=PostResponseItem)
