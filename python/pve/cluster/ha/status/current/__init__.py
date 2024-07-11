from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


@dataclass
class CurrentClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'current'}"

    def get(self) -> list[dict[str, Any]]:
        """
        Get HA manger status.
        """
        return self.client.get(self.path, parse_as=list[dict[str, Any]])


@dataclass
class AsyncCurrentClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'current'}"

    async def get(self) -> list[dict[str, Any]]:
        """
        Get HA manger status.
        """
        return await self.client.get(self.path, parse_as=list[dict[str, Any]])
