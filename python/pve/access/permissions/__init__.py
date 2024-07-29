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
    """
    Map of "path" => (Map of "privilege" => "propagate boolean").
    """

    pass


class GetParameters(BaseModel):
    # Only dump this specific path, not the whole tree.
    path: Optional[str] = Field(default=None)
    # User ID or full API token ID
    userid: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class PermissionsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'permissions'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Retrieve effective permissions of given user/token.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncPermissionsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'permissions'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Retrieve effective permissions of given user/token.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
