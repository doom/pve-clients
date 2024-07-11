from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    port: Optional[str] = Field(default=None)
    socket: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # unix socket to forward to
    socket: str
    # ticket return by initial 'mtunnel' API call, or retrieved via 'ticket' tunnel command
    ticket: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MtunnelwebsocketClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'mtunnelwebsocket'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Migration tunnel endpoint for websocket upgrade - only for internal use by VM migration.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncMtunnelwebsocketClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'mtunnelwebsocket'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Migration tunnel endpoint for websocket upgrade - only for internal use by VM migration.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
