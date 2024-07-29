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
    socket: str
    ticket: str
    upid: str

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # List of network bridges to check availability. Will be checked again for actually used bridges during migration.
    bridges: Optional[str] = Field(default=None)
    # List of storages to check permission and availability. Will be checked again for all actually used storages during migration.
    storages: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MtunnelClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'mtunnel'}"

    def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Migration tunnel endpoint - only for internal use by VM migration.
        """
        return self.client.post(self.path, parameters, parse_as=PostResponseItem)


@dataclass
class AsyncMtunnelClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'mtunnel'}"

    async def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Migration tunnel endpoint - only for internal use by VM migration.
        """
        return await self.client.post(self.path, parameters, parse_as=PostResponseItem)
