from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import tokenid as _tokenid


class GetResponseItem(BaseModel):
    comment: Optional[str] = Field(default=None)
    # API token expiration date (seconds since epoch). '0' means no expiration date.
    expire: Optional[int] = Field(default=None)
    # Restrict API token privileges with separate ACLs (default), or give full privileges of corresponding user.
    privsep: Optional[bool] = Field(default=None)
    # User-specific token identifier.
    tokenid: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class TokenClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'token'}"

    def tokenid(self, tokenid: str) -> _tokenid.TokenidClient:
        return _tokenid.TokenidClient(
            self.client,
            self.path,
            tokenid,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Get user API tokens.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncTokenClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'token'}"

    def tokenid(self, tokenid: str) -> _tokenid.AsyncTokenidClient:
        return _tokenid.AsyncTokenidClient(
            self.client,
            self.path,
            tokenid,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Get user API tokens.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
