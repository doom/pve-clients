from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetParameters(BaseModel):
    # Volume identifier
    volume: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ExtractconfigClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'extractconfig'}"

    def get(self, parameters: GetParameters) -> str:
        """
        Extract configuration from vzdump backup archive.
        """
        return self.client.get(self.path, parameters, parse_as=str)


@dataclass
class AsyncExtractconfigClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'extractconfig'}"

    async def get(self, parameters: GetParameters) -> str:
        """
        Extract configuration from vzdump backup archive.
        """
        return await self.client.get(self.path, parameters, parse_as=str)
