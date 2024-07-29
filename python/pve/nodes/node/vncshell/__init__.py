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
    cert: str
    port: int
    ticket: str
    upid: str
    user: str

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # Run specific command or default to login.
    cmd: Optional[str] = Field(default=None)
    # Add parameters to a command. Encoded as null terminated strings.
    cmd_opts: Optional[str] = Field(alias="cmd-opts", default=None)
    # sets the height of the console in pixels.
    height: Optional[int] = Field(default=None)
    # use websocket instead of standard vnc.
    websocket: Optional[bool] = Field(default=None)
    # sets the width of the console in pixels.
    width: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class VncshellClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'vncshell'}"

    def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Creates a VNC Shell proxy.
        """
        return self.client.post(self.path, parameters, parse_as=PostResponseItem)


@dataclass
class AsyncVncshellClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'vncshell'}"

    async def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Creates a VNC Shell proxy.
        """
        return await self.client.post(self.path, parameters, parse_as=PostResponseItem)
