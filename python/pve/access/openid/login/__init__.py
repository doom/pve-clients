from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class Cap(BaseModel):
    pass


class PostResponseItem(BaseModel):
    csrfprevention_token: str = Field(alias="CSRFPreventionToken")
    cap: Cap
    clustername: Optional[str] = Field(default=None)
    ticket: str
    username: str

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # OpenId authorization code.
    code: str
    # Redirection Url. The client should set this to the used server url (location.origin).
    redirect_url: str = Field(alias="redirect-url")
    # OpenId state.
    state: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class LoginClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'login'}"

    def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Verify OpenID authorization code and create a ticket.
        """
        return self.client.post(self.path, parameters, parse_as=PostResponseItem)


@dataclass
class AsyncLoginClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'login'}"

    async def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Verify OpenID authorization code and create a ticket.
        """
        return await self.client.post(self.path, parameters, parse_as=PostResponseItem)
