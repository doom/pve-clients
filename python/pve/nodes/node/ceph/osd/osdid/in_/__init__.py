from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


@dataclass
class InClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'in'}"

    def post(self):
        """
        ceph osd in
        """
        return self.client.post(
            self.path,
        )


@dataclass
class AsyncInClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'in'}"

    async def post(self):
        """
        ceph osd in
        """
        return await self.client.post(
            self.path,
        )
