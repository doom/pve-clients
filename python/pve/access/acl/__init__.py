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
    # Remove permissions (instead of adding it).
    delete: Optional[bool] = Field(default=None)
    # List of groups.
    groups: Optional[str] = Field(default=None)
    # Access control path
    path: str
    # Allow to propagate (inherit) permissions.
    propagate: Optional[bool] = Field(default=None)
    # List of roles.
    roles: str
    # List of API tokens.
    tokens: Optional[str] = Field(default=None)
    # List of users.
    users: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Access control path
    path: str
    # Allow to propagate (inherit) permissions.
    propagate: Optional[bool] = Field(default=None)
    roleid: str
    type: str
    ugid: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class AclClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'acl'}"

    def get(self) -> list[GetResponseItem]:
        """
        Get Access Control List (ACLs).
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def put(self, parameters: PutParameters):
        """
        Update Access Control List (add or remove permissions).
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncAclClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'acl'}"

    async def get(self) -> list[GetResponseItem]:
        """
        Get Access Control List (ACLs).
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def put(self, parameters: PutParameters):
        """
        Update Access Control List (add or remove permissions).
        """
        return await self.client.put(self.path, parameters)
