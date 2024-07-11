from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import name as _name


class PostParameters(BaseModel):
    # Configure storage using the zpool.
    add_storage: Optional[bool] = Field(default=None)
    # Pool sector size exponent.
    ashift: Optional[int] = Field(default=None)
    # The compression algorithm to use.
    compression: Optional[str] = Field(default=None)
    # The block devices you want to create the zpool on.
    devices: str
    draid_config: Optional[str] = Field(alias="draid-config", default=None)
    # The storage identifier.
    name: str
    # The RAID level to use.
    raidlevel: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    #
    alloc: int
    #
    dedup: float
    #
    frag: int
    #
    free: int
    #
    health: str
    #
    name: str
    #
    size: int

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ZfsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'zfs'}"

    def name(self, name: str) -> _name.NameClient:
        return _name.NameClient(
            self.client,
            self.path,
            name,
        )

    def get(self) -> list[GetResponseItem]:
        """
        List Zpools.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters) -> str:
        """
        Create a ZFS pool.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncZfsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'zfs'}"

    def name(self, name: str) -> _name.AsyncNameClient:
        return _name.AsyncNameClient(
            self.client,
            self.path,
            name,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        List Zpools.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters) -> str:
        """
        Create a ZFS pool.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
