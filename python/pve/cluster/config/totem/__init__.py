from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    pass


@dataclass
class TotemClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'totem'}"

    def get(self) -> GetResponseItem:
        """
        Get corosync totem protocol settings.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)


@dataclass
class AsyncTotemClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'totem'}"

    async def get(self) -> GetResponseItem:
        """
        Get corosync totem protocol settings.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)
