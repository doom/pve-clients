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
    comment: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    nomatch: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


class DeleteParameters(BaseModel):
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class CidrClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, cidr: str):
        self.client = client
        self.path = f"{parent_path}/{cidr}"

    def delete(self, parameters: DeleteParameters):
        """
        Remove IP or Network from IPSet.
        """
        return self.client.delete(self.path, parameters)

    def get(self) -> GetResponseItem:
        """
        Read IP or Network settings from IPSet.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update IP or Network settings
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncCidrClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, cidr: str):
        self.client = client
        self.path = f"{parent_path}/{cidr}"

    async def delete(self, parameters: DeleteParameters):
        """
        Remove IP or Network from IPSet.
        """
        return await self.client.delete(self.path, parameters)

    async def get(self) -> GetResponseItem:
        """
        Read IP or Network settings from IPSet.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update IP or Network settings
        """
        return await self.client.put(self.path, parameters)
