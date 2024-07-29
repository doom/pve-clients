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
class ReloadClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'reload'}"

    def post(self) -> str:
        """
        Reload service. Falls back to restart if service cannot be reloaded.
        """
        return self.client.post(self.path, parse_as=str)


@dataclass
class AsyncReloadClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'reload'}"

    async def post(self) -> str:
        """
        Reload service. Falls back to restart if service cannot be reloaded.
        """
        return await self.client.post(self.path, parse_as=str)
