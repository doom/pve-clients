from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetParameters(BaseModel):
    # Config type.
    type: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class DumpClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'dump'}"

    def get(self, parameters: GetParameters) -> str:
        """
        Get automatically generated cloudinit config.
        """
        return self.client.get(self.path, parameters, parse_as=str)


@dataclass
class AsyncDumpClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'dump'}"

    async def get(self, parameters: GetParameters) -> str:
        """
        Get automatically generated cloudinit config.
        """
        return await self.client.get(self.path, parameters, parse_as=str)
