from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


@dataclass
class IdClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, id: str):
        self.client = client
        self.path = f"{parent_path}/{id}"

    def delete(self) -> str:
        """
        Destroy Ceph Manager.
        """
        return self.client.delete(self.path, parse_as=str)

    def post(self) -> str:
        """
        Create Ceph Manager
        """
        return self.client.post(self.path, parse_as=str)


@dataclass
class AsyncIdClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, id: str):
        self.client = client
        self.path = f"{parent_path}/{id}"

    async def delete(self) -> str:
        """
        Destroy Ceph Manager.
        """
        return await self.client.delete(self.path, parse_as=str)

    async def post(self) -> str:
        """
        Create Ceph Manager
        """
        return await self.client.post(self.path, parse_as=str)
