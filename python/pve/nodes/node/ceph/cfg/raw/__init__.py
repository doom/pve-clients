from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


@dataclass
class RawClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'raw'}"

    def get(self) -> str:
        """
        Get the Ceph configuration file.
        """
        return self.client.get(self.path, parse_as=str)


@dataclass
class AsyncRawClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'raw'}"

    async def get(self) -> str:
        """
        Get the Ceph configuration file.
        """
        return await self.client.get(self.path, parse_as=str)
