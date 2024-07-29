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
    pass


@dataclass
class NetstatClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'netstat'}"

    def get(self) -> list[GetResponseItem]:
        """
        Read tap/vm network device interface counters
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncNetstatClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'netstat'}"

    async def get(self) -> list[GetResponseItem]:
        """
        Read tap/vm network device interface counters
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
