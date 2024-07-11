from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


@dataclass
class ResumeClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'resume'}"

    def post(self) -> str:
        """
        Resume the container.
        """
        return self.client.post(self.path, parse_as=str)


@dataclass
class AsyncResumeClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'resume'}"

    async def post(self) -> str:
        """
        Resume the container.
        """
        return await self.client.post(self.path, parse_as=str)
