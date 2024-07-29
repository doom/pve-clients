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
    # Returned if requested with 'generate-password' param. Consists of printable ASCII characters ('!' .. '~').
    password: Optional[str] = Field(default=None)
    port: int
    ticket: str
    upid: str
    user: str

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # Generates a random password to be used as ticket instead of the API ticket.
    generate_password: Optional[bool] = Field(alias="generate-password", default=None)
    # Prepare for websocket upgrade (only required when using serial terminal, otherwise upgrade is always possible).
    websocket: Optional[bool] = Field(default=None)

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
