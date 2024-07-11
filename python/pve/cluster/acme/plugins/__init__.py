from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import id as _id


class PostParameters(BaseModel):
    # API plugin name
    api: Optional[str] = Field(default=None)
    # DNS plugin data. (base64 encoded)
    data: Optional[str] = Field(default=None)
    # Flag to disable the config.
    disable: Optional[bool] = Field(default=None)
    # ACME Plugin ID name
    id: str
    # List of cluster node names.
    nodes: Optional[str] = Field(default=None)
    # ACME challenge type.
    type: str
    # Extra delay in seconds to wait before requesting validation. Allows to cope with a long TTL of DNS records.
    validation_delay: Optional[int] = Field(alias="validation-delay", default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Unique identifier for ACME plugin instance.
    plugin: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Only list ACME plugins of a specific type
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class PluginsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'plugins'}"

    def id(self, id: str) -> _id.IdClient:
        return _id.IdClient(
            self.client,
            self.path,
            id,
        )

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        ACME plugin index.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Add ACME plugin configuration.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncPluginsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'plugins'}"

    def id(self, id: str) -> _id.AsyncIdClient:
        return _id.AsyncIdClient(
            self.client,
            self.path,
            id,
        )

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        ACME plugin index.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )

    async def post(self, parameters: PostParameters):
        """
        Add ACME plugin configuration.
        """
        return await self.client.post(self.path, parameters)
