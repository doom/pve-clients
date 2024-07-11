from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


@dataclass
class TemplateClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'template'}"

    def post(self):
        """
        Create a Template.
        """
        return self.client.post(
            self.path,
        )


@dataclass
class AsyncTemplateClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'template'}"

    async def post(self):
        """
        Create a Template.
        """
        return await self.client.post(
            self.path,
        )
