from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PutParameters(BaseModel):
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    key: Optional[str] = Field(default=None)
    reversemaskv6: Optional[int] = Field(default=None)
    ttl: Optional[int] = Field(default=None)
    url: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


@dataclass
class DnsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, dns: str):
        self.client = client
        self.path = f"{parent_path}/{dns}"

    def delete(self):
        """
        Delete sdn dns object configuration.
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> GetResponseItem:
        """
        Read sdn dns configuration.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update sdn dns object configuration.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncDnsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, dns: str):
        self.client = client
        self.path = f"{parent_path}/{dns}"

    async def delete(self):
        """
        Delete sdn dns object configuration.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> GetResponseItem:
        """
        Read sdn dns configuration.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update sdn dns object configuration.
        """
        return await self.client.put(self.path, parameters)
