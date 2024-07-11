from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostResponseItem(BaseModel):
    """
    Returns an object with a single `result` property.
    """

    pass


@dataclass
class SuspendHybridClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'suspend-hybrid'}"

    def post(self) -> PostResponseItem:
        """
        Execute suspend-hybrid.
        """
        return self.client.post(self.path, parse_as=PostResponseItem)


@dataclass
class AsyncSuspendHybridClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'suspend-hybrid'}"

    async def post(self) -> PostResponseItem:
        """
        Execute suspend-hybrid.
        """
        return await self.client.post(self.path, parse_as=PostResponseItem)
