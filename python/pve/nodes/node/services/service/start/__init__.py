from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


@dataclass
class StartClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'start'}"

    def post(self) -> str:
        """
        Start service.
        """
        return self.client.post(self.path, parse_as=str)


@dataclass
class AsyncStartClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'start'}"

    async def post(self) -> str:
        """
        Start service.
        """
        return await self.client.post(self.path, parse_as=str)
