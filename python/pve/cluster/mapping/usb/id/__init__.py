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
    # Description of the logical USB device.
    description: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # A list of maps for the cluster nodes.
    map: list[str]

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


@dataclass
class IdClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, id: str):
        self.client = client
        self.path = f"{parent_path}/{id}"

    def delete(self):
        """
        Remove Hardware Mapping.
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> GetResponseItem:
        """
        Get USB Mapping.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update a hardware mapping.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncIdClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, id: str):
        self.client = client
        self.path = f"{parent_path}/{id}"

    async def delete(self):
        """
        Remove Hardware Mapping.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> GetResponseItem:
        """
        Get USB Mapping.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update a hardware mapping.
        """
        return await self.client.put(self.path, parameters)
