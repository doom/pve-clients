from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import name as _name


class PostParameters(BaseModel):
    # Comment
    comment: Optional[str] = Field(default=None)
    # Disable this target
    disable: Optional[bool] = Field(default=None)
    # The name of the endpoint.
    name: str
    # Server URL
    server: str
    # Secret token
    token: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Comment
    comment: Optional[str] = Field(default=None)
    # Disable this target
    disable: Optional[bool] = Field(default=None)
    # The name of the endpoint.
    name: str
    # Show if this entry was created by a user or was built-in
    origin: str
    # Server URL
    server: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class GotifyClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'gotify'}"

    def name(self, name: str) -> _name.NameClient:
        return _name.NameClient(
            self.client,
            self.path,
            name,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Returns a list of all gotify endpoints
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create a new gotify endpoint
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncGotifyClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'gotify'}"

    def name(self, name: str) -> _name.AsyncNameClient:
        return _name.AsyncNameClient(
            self.client,
            self.path,
            name,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Returns a list of all gotify endpoints
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters):
        """
        Create a new gotify endpoint
        """
        return await self.client.post(self.path, parameters)
