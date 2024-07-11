from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import name as _name


class PostParameters(BaseModel):
    # Configure storage using the thinpool.
    add_storage: Optional[bool] = Field(default=None)
    # The block device you want to create the thinpool on.
    device: str
    # The storage identifier.
    name: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # The name of the thinpool.
    lv: str
    # The size of the thinpool in bytes.
    lv_size: int
    # The size of the metadata lv in bytes.
    metadata_size: int
    # The used bytes of the metadata lv.
    metadata_used: int
    # The used bytes of the thinpool.
    used: int
    # The associated volume group.
    vg: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class LvmthinClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'lvmthin'}"

    def name(self, name: str) -> _name.NameClient:
        return _name.NameClient(
            self.client,
            self.path,
            name,
        )

    def get(self) -> list[GetResponseItem]:
        """
        List LVM thinpools
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters) -> str:
        """
        Create an LVM thinpool
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncLvmthinClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'lvmthin'}"

    def name(self, name: str) -> _name.AsyncNameClient:
        return _name.AsyncNameClient(
            self.client,
            self.path,
            name,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        List LVM thinpools
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters) -> str:
        """
        Create an LVM thinpool
        """
        return await self.client.post(self.path, parameters, parse_as=str)
