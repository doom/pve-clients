from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    port: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Port number returned by previous vncproxy call.
    port: int
    # Ticket from previous call to vncproxy.
    vncticket: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class VncwebsocketClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'vncwebsocket'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Opens a websocket for VNC traffic.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncVncwebsocketClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'vncwebsocket'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Opens a websocket for VNC traffic.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
