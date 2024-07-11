from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostResponseItem(BaseModel):
    cert: str
    port: int
    ticket: str
    upid: str
    user: str

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # sets the height of the console in pixels.
    height: Optional[int] = Field(default=None)
    # use websocket instead of standard VNC.
    websocket: Optional[bool] = Field(default=None)
    # sets the width of the console in pixels.
    width: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class VncproxyClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'vncproxy'}"

    def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Creates a TCP VNC proxy connections.
        """
        return self.client.post(self.path, parameters, parse_as=PostResponseItem)


@dataclass
class AsyncVncproxyClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'vncproxy'}"

    async def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Creates a TCP VNC proxy connections.
        """
        return await self.client.post(self.path, parameters, parse_as=PostResponseItem)
