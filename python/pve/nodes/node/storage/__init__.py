from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import storage as _storage


class GetResponseItem(BaseModel):
    # Set when storage is accessible.
    active: Optional[bool] = Field(default=None)
    # Available storage space in bytes.
    avail: Optional[int] = Field(default=None)
    # Allowed storage content types.
    content: str
    # Set when storage is enabled (not disabled).
    enabled: Optional[bool] = Field(default=None)
    # Shared flag from storage configuration.
    shared: Optional[bool] = Field(default=None)
    # The storage identifier.
    storage: str
    # Total storage space in bytes.
    total: Optional[int] = Field(default=None)
    # Storage type.
    type: str
    # Used storage space in bytes.
    used: Optional[int] = Field(default=None)
    # Used fraction (used/total).
    used_fraction: Optional[float] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Only list stores which support this content type.
    content: Optional[str] = Field(default=None)
    # Only list stores which are enabled (not disabled in config).
    enabled: Optional[bool] = Field(default=None)
    # Include information about formats
    format: Optional[bool] = Field(default=None)
    # Only list status for  specified storage
    storage: Optional[str] = Field(default=None)
    # If target is different to 'node', we only lists shared storages which content is accessible on this 'node' and the specified 'target' node.
    target: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class StorageClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'storage'}"

    def storage(self, storage: str) -> _storage.StorageClient:
        return _storage.StorageClient(
            self.client,
            self.path,
            storage,
        )

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Get status for all datastores.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncStorageClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'storage'}"

    def storage(self, storage: str) -> _storage.AsyncStorageClient:
        return _storage.AsyncStorageClient(
            self.client,
            self.path,
            storage,
        )

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Get status for all datastores.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
