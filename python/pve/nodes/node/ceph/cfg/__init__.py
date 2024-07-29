from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import db as _db
from . import raw as _raw


class GetResponseItem(BaseModel):
    pass


@dataclass
class CfgClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'cfg'}"

    def raw(self) -> _raw.RawClient:
        return _raw.RawClient(
            self.client,
            self.path,
        )

    def db(self) -> _db.DbClient:
        return _db.DbClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncCfgClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'cfg'}"

    def raw(self) -> _raw.AsyncRawClient:
        return _raw.AsyncRawClient(
            self.client,
            self.path,
        )

    def db(self) -> _db.AsyncDbClient:
        return _db.AsyncDbClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
