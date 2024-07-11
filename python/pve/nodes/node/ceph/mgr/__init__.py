from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import id as _id


class GetResponseItem(BaseModel):
    addr: Optional[str] = Field(default=None)
    host: Optional[str] = Field(default=None)
    # The name (ID) for the MGR
    name: dict[str, Any]
    # State of the MGR
    state: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MgrClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'mgr'}"

    def id(self, id: str) -> _id.IdClient:
        return _id.IdClient(
            self.client,
            self.path,
            id,
        )

    def get(self) -> list[GetResponseItem]:
        """
        MGR directory index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncMgrClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'mgr'}"

    def id(self, id: str) -> _id.AsyncIdClient:
        return _id.AsyncIdClient(
            self.client,
            self.path,
            id,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        MGR directory index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
