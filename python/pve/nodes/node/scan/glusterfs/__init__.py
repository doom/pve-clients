from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetResponseItem(BaseModel):
    # The volume name.
    volname: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # The server address (name or IP).
    server: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class GlusterfsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'glusterfs'}"

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Scan remote GlusterFS server.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncGlusterfsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'glusterfs'}"

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Scan remote GlusterFS server.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
