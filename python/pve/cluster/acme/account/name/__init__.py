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
    # Contact email addresses.
    contact: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class Account(BaseModel):
    pass


class GetResponseItem(BaseModel):
    account: Optional[Account] = Field(default=None)
    # URL of ACME CA directory endpoint.
    directory: Optional[str] = Field(default=None)
    location: Optional[str] = Field(default=None)
    tos: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class NameClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    def delete(self) -> str:
        """
        Deactivate existing ACME account at CA.
        """
        return self.client.delete(self.path, parse_as=str)

    def get(self) -> GetResponseItem:
        """
        Return existing ACME account information.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters) -> str:
        """
        Update existing ACME account information with CA. Note: not specifying any new account information triggers a refresh.
        """
        return self.client.put(self.path, parameters, parse_as=str)


@dataclass
class AsyncNameClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    async def delete(self) -> str:
        """
        Deactivate existing ACME account at CA.
        """
        return await self.client.delete(self.path, parse_as=str)

    async def get(self) -> GetResponseItem:
        """
        Return existing ACME account information.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters) -> str:
        """
        Update existing ACME account information with CA. Note: not specifying any new account information triggers a refresh.
        """
        return await self.client.put(self.path, parameters, parse_as=str)
