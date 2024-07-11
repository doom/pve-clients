from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    # base64 path of the current entry
    filepath: str
    # If this entry is a leaf in the directory graph.
    leaf: bool
    # Entry last-modified time (unix timestamp).
    mtime: Optional[int] = Field(default=None)
    # Entry file size.
    size: Optional[int] = Field(default=None)
    # Entry display text.
    text: str
    # Entry type.
    type: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # base64-path to the directory or file being listed, or \"/\".
    filepath: str
    # Backup volume ID or name. Currently only PBS snapshots are supported.
    volume: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ListClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'list'}"

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List files and directories for single file restore under the given path.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncListClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'list'}"

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List files and directories for single file restore under the given path.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
