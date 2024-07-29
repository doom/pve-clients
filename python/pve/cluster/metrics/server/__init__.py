from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import id as _id


class GetResponseItem(BaseModel):
    # Flag to disable the plugin.
    disable: bool
    # The ID of the entry.
    id: str
    # Server network port
    port: int
    # Server dns name or IP address
    server: str
    # Plugin type.
    type: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ServerClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'server'}"

    def id(self, id: str) -> _id.IdClient:
        return _id.IdClient(
            self.client,
            self.path,
            id,
        )

    def get(self) -> list[GetResponseItem]:
        """
        List configured metric servers.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncServerClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'server'}"

    def id(self, id: str) -> _id.AsyncIdClient:
        return _id.AsyncIdClient(
            self.client,
            self.path,
            id,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        List configured metric servers.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
