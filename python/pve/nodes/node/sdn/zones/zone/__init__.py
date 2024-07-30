from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import content as _content


class GetResponseItem(BaseModel):
    subdir: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ZoneClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, zone: str):
        self.client = client
        self.path = f"{parent_path}/{zone}"

    def content(self) -> _content.ContentClient:
        return _content.ContentClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncZoneClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, zone: str):
        self.client = client
        self.path = f"{parent_path}/{zone}"

    def content(self) -> _content.AsyncContentClient:
        return _content.AsyncContentClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
