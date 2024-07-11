from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


@dataclass
class StopClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'stop'}"

    def post(self) -> str:
        """
        Stop service.
        """
        return self.client.post(self.path, parse_as=str)


@dataclass
class AsyncStopClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'stop'}"

    async def post(self) -> str:
        """
        Stop service.
        """
        return await self.client.post(self.path, parse_as=str)
