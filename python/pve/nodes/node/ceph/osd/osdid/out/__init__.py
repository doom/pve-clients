from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


@dataclass
class OutClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'out'}"

    def post(self):
        """
        ceph osd out
        """
        return self.client.post(
            self.path,
        )


@dataclass
class AsyncOutClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'out'}"

    async def post(self):
        """
        ceph osd out
        """
        return await self.client.post(
            self.path,
        )
