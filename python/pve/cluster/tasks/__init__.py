from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    upid: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class TasksClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'tasks'}"

    def get(self) -> list[GetResponseItem]:
        """
        List recent tasks (cluster wide).
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncTasksClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'tasks'}"

    async def get(self) -> list[GetResponseItem]:
        """
        List recent tasks (cluster wide).
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
