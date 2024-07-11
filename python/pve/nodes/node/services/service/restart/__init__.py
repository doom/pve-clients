from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


@dataclass
class RestartClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'restart'}"

    def post(self) -> str:
        """
        Hard restart service. Use reload if you want to reduce interruptions.
        """
        return self.client.post(self.path, parse_as=str)


@dataclass
class AsyncRestartClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'restart'}"

    async def post(self) -> str:
        """
        Hard restart service. Use reload if you want to reduce interruptions.
        """
        return await self.client.post(self.path, parse_as=str)
