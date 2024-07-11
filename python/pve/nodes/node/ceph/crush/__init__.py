from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


@dataclass
class CrushClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'crush'}"

    def get(self) -> str:
        """
        Get OSD crush map
        """
        return self.client.get(self.path, parse_as=str)


@dataclass
class AsyncCrushClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'crush'}"

    async def get(self) -> str:
        """
        Get OSD crush map
        """
        return await self.client.get(self.path, parse_as=str)
