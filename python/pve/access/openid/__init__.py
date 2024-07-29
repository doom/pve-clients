from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import auth_url as _auth_url
from . import login as _login


class GetResponseItem(BaseModel):
    subdir: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class OpenidClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'openid'}"

    def auth_url(self) -> _auth_url.AuthUrlClient:
        return _auth_url.AuthUrlClient(
            self.client,
            self.path,
        )

    def login(self) -> _login.LoginClient:
        return _login.LoginClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncOpenidClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'openid'}"

    def auth_url(self) -> _auth_url.AsyncAuthUrlClient:
        return _auth_url.AsyncAuthUrlClient(
            self.client,
            self.path,
        )

    def login(self) -> _login.AsyncLoginClient:
        return _login.AsyncLoginClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
