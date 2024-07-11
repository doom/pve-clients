from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostResponseItem(BaseModel):
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

    class Config(CommonPydanticConfig):
        pass


@dataclass
class TermproxyClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'termproxy'}"

    def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Creates a VNC Shell proxy.
        """
        return self.client.post(self.path, parameters, parse_as=PostResponseItem)


@dataclass
class AsyncTermproxyClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'termproxy'}"

    async def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Creates a VNC Shell proxy.
        """
        return await self.client.post(self.path, parameters, parse_as=PostResponseItem)
