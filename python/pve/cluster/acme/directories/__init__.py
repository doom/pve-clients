from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    name: str
    # URL of ACME CA directory endpoint.
    url: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class DirectoriesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'directories'}"

    def get(self) -> list[GetResponseItem]:
        """
        Get named known ACME directory endpoints.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncDirectoriesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'directories'}"

    async def get(self) -> list[GetResponseItem]:
        """
        Get named known ACME directory endpoints.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
