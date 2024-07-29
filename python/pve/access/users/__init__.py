from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import userid as _userid


class PostParameters(BaseModel):
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
    # Initial password.
    password: Optional[str] = Field(default=None)
    # Full User ID, in the `name@realm` format.
    userid: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItemTokensItem(BaseModel):
    comment: Optional[str] = Field(default=None)
    # API token expiration date (seconds since epoch). '0' means no expiration date.
    expire: Optional[int] = Field(default=None)
    # Restrict API token privileges with separate ACLs (default), or give full privileges of corresponding user.
    privsep: Optional[bool] = Field(default=None)
    # User-specific token identifier.
    tokenid: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
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
    # The type of the users realm
    realm_type: Optional[str] = Field(alias="realm-type", default=None)
    tokens: Optional[list[GetResponseItemTokensItem]] = Field(default=None)
    # Full User ID, in the `name@realm` format.
    userid: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Optional filter for enable property.
    enabled: Optional[bool] = Field(default=None)
    # Include group and token information.
    full: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class UsersClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'users'}"

    def userid(self, userid: str) -> _userid.UseridClient:
        return _userid.UseridClient(
            self.client,
            self.path,
            userid,
        )

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        User index.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create new user.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncUsersClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'users'}"

    def userid(self, userid: str) -> _userid.AsyncUseridClient:
        return _userid.AsyncUseridClient(
            self.client,
            self.path,
            userid,
        )

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        User index.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )

    async def post(self, parameters: PostParameters):
        """
        Create new user.
        """
        return await self.client.post(self.path, parameters)
