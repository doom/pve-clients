from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import snapname as _snapname


class PostParameters(BaseModel):
    # A textual description or comment.
    description: Optional[str] = Field(default=None)
    # The name of the snapshot.
    snapname: str
    # Save the vmstate
    vmstate: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Snapshot description.
    description: str
    # Snapshot identifier. Value 'current' identifies the current VM.
    name: str
    # Parent snapshot identifier.
    parent: Optional[str] = Field(default=None)
    # Snapshot creation time
    snaptime: Optional[int] = Field(default=None)
    # Snapshot includes RAM.
    vmstate: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class SnapshotClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'snapshot'}"

    def snapname(self, snapname: str) -> _snapname.SnapnameClient:
        return _snapname.SnapnameClient(
            self.client,
            self.path,
            snapname,
        )

    def get(self) -> list[GetResponseItem]:
        """
        List all snapshots.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters) -> str:
        """
        Snapshot a VM.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncSnapshotClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'snapshot'}"

    def snapname(self, snapname: str) -> _snapname.AsyncSnapnameClient:
        return _snapname.AsyncSnapnameClient(
            self.client,
            self.path,
            snapname,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        List all snapshots.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters) -> str:
        """
        Snapshot a VM.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
