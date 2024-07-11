from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetParameters(BaseModel):
    # Package name.
    name: str
    # Package version.
    version: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ChangelogClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'changelog'}"

    def get(self, parameters: GetParameters) -> str:
        """
        Get package changelogs.
        """
        return self.client.get(self.path, parameters, parse_as=str)


@dataclass
class AsyncChangelogClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'changelog'}"

    async def get(self, parameters: GetParameters) -> str:
        """
        Get package changelogs.
        """
        return await self.client.get(self.path, parameters, parse_as=str)
