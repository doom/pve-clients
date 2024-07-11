from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    """
    Returns an object with a single `result` property.
    """

    pass


@dataclass
class InfoClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'info'}"

    def get(self) -> GetResponseItem:
        """
        Execute info.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)


@dataclass
class AsyncInfoClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'info'}"

    async def get(self) -> GetResponseItem:
        """
        Execute info.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)
