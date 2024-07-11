from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    pass


@dataclass
class StatusClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'status'}"

    def get(self) -> GetResponseItem:
        """
        Get replication job status.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)


@dataclass
class AsyncStatusClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'status'}"

    async def get(self) -> GetResponseItem:
        """
        Get replication job status.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)
