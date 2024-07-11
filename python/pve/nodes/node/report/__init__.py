from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


@dataclass
class ReportClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'report'}"

    def get(self) -> str:
        """
        Gather various systems information about a node
        """
        return self.client.get(self.path, parse_as=str)


@dataclass
class AsyncReportClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'report'}"

    async def get(self) -> str:
        """
        Gather various systems information about a node
        """
        return await self.client.get(self.path, parse_as=str)
