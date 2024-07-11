from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import name as _name


class GetResponseItem(BaseModel):
    addr: Optional[str] = Field(default=None)
    host: Optional[str] = Field(default=None)
    # The name (ID) for the MDS
    name: dict[str, Any]
    rank: Optional[int] = Field(default=None)
    # If true, the standby MDS is polling the active MDS for faster recovery (hot standby).
    standby_replay: Optional[bool] = Field(default=None)
    # State of the MDS
    state: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MdsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'mds'}"

    def name(self, name: str) -> _name.NameClient:
        return _name.NameClient(
            self.client,
            self.path,
            name,
        )

    def get(self) -> list[GetResponseItem]:
        """
        MDS directory index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncMdsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'mds'}"

    def name(self, name: str) -> _name.AsyncNameClient:
        return _name.AsyncNameClient(
            self.client,
            self.path,
            name,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        MDS directory index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
