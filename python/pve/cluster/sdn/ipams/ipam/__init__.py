from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import status as _status


class PutParameters(BaseModel):
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    section: Optional[int] = Field(default=None)
    token: Optional[str] = Field(default=None)
    url: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


@dataclass
class IpamClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, ipam: str):
        self.client = client
        self.path = f"{parent_path}/{ipam}"

    def status(self) -> _status.StatusClient:
        return _status.StatusClient(
            self.client,
            self.path,
        )

    def delete(self):
        """
        Delete sdn ipam object configuration.
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> GetResponseItem:
        """
        Read sdn ipam configuration.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update sdn ipam object configuration.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncIpamClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, ipam: str):
        self.client = client
        self.path = f"{parent_path}/{ipam}"

    def status(self) -> _status.AsyncStatusClient:
        return _status.AsyncStatusClient(
            self.client,
            self.path,
        )

    async def delete(self):
        """
        Delete sdn ipam object configuration.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> GetResponseItem:
        """
        Read sdn ipam configuration.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update sdn ipam object configuration.
        """
        return await self.client.put(self.path, parameters)
