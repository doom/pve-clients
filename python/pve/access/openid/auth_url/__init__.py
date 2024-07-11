from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # Authentication domain ID
    realm: str
    # Redirection Url. The client should set this to the used server url (location.origin).
    redirect_url: str = Field(alias="redirect-url")

    class Config(CommonPydanticConfig):
        pass


@dataclass
class AuthUrlClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'auth-url'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Get the OpenId Authorization Url for the specified realm.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncAuthUrlClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'auth-url'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Get the OpenId Authorization Url for the specified realm.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
