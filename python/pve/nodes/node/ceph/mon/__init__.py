from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import monid as _monid


class GetResponseItem(BaseModel):
    addr: Optional[str] = Field(default=None)
    ceph_version: Optional[str] = Field(default=None)
    ceph_version_short: Optional[str] = Field(default=None)
    direxists: Optional[str] = Field(default=None)
    host: Optional[bool] = Field(default=None)
    name: str
    quorum: Optional[bool] = Field(default=None)
    rank: Optional[int] = Field(default=None)
    service: Optional[int] = Field(default=None)
    state: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MonClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'mon'}"

    def monid(self, monid: str) -> _monid.MonidClient:
        return _monid.MonidClient(
            self.client,
            self.path,
            monid,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Get Ceph monitor list.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncMonClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'mon'}"

    def monid(self, monid: str) -> _monid.AsyncMonidClient:
        return _monid.AsyncMonidClient(
            self.client,
            self.path,
            monid,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Get Ceph monitor list.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
