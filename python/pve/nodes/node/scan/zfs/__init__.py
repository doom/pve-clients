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
    # ZFS pool name.
    pool: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ZfsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'zfs'}"

    def get(self) -> list[GetResponseItem]:
        """
        Scan zfs pool list on local node.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncZfsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'zfs'}"

    async def get(self) -> list[GetResponseItem]:
        """
        Scan zfs pool list on local node.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
