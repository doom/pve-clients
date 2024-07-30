from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import tfa as _tfa
from . import token as _token
from . import unlock_tfa as _unlock_tfa


class PutParameters(BaseModel):
    append: Optional[bool] = Field(default=None)
    comment: Optional[str] = Field(default=None)
    email: Optional[str] = Field(default=None)
    # Enable the account (default). You can set this to '0' to disable the account
    enable: Optional[bool] = Field(default=None)
    # Account expiration date (seconds since epoch). '0' means no expiration date.
    expire: Optional[int] = Field(default=None)
    firstname: Optional[str] = Field(default=None)
    groups: Optional[str] = Field(default=None)
    # Keys for two factor auth (yubico).
    keys: Optional[str] = Field(default=None)
    lastname: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class Tokens(BaseModel):
    pass


class GetResponseItem(BaseModel):
    comment: Optional[str] = Field(default=None)
    email: Optional[str] = Field(default=None)
    # Enable the account (default). You can set this to '0' to disable the account
    enable: Optional[bool] = Field(default=None)
    # Account expiration date (seconds since epoch). '0' means no expiration date.
    expire: Optional[int] = Field(default=None)
    firstname: Optional[str] = Field(default=None)
    groups: Optional[list[str]] = Field(default=None)
    # Keys for two factor auth (yubico).
    keys: Optional[str] = Field(default=None)
    lastname: Optional[str] = Field(default=None)
    tokens: Optional[Tokens] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class UseridClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, userid: str):
        self.client = client
        self.path = f"{parent_path}/{userid}"

    def tfa(self) -> _tfa.TfaClient:
        return _tfa.TfaClient(
            self.client,
            self.path,
        )

    def unlock_tfa(self) -> _unlock_tfa.UnlockTfaClient:
        return _unlock_tfa.UnlockTfaClient(
            self.client,
            self.path,
        )

    def token(self) -> _token.TokenClient:
        return _token.TokenClient(
            self.client,
            self.path,
        )

    def delete(self):
        """
        Delete user.
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> GetResponseItem:
        """
        Get user configuration.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update user configuration.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncUseridClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, userid: str):
        self.client = client
        self.path = f"{parent_path}/{userid}"

    def tfa(self) -> _tfa.AsyncTfaClient:
        return _tfa.AsyncTfaClient(
            self.client,
            self.path,
        )

    def unlock_tfa(self) -> _unlock_tfa.AsyncUnlockTfaClient:
        return _unlock_tfa.AsyncUnlockTfaClient(
            self.client,
            self.path,
        )

    def token(self) -> _token.AsyncTokenClient:
        return _token.AsyncTokenClient(
            self.client,
            self.path,
        )

    async def delete(self):
        """
        Delete user.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> GetResponseItem:
        """
        Get user configuration.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update user configuration.
        """
        return await self.client.put(self.path, parameters)
