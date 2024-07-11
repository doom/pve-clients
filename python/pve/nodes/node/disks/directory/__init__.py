from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import name as _name


class PostParameters(BaseModel):
    # Configure storage using the directory.
    add_storage: Optional[bool] = Field(default=None)
    # The block device you want to create the filesystem on.
    device: str
    # The desired filesystem.
    filesystem: Optional[str] = Field(default=None)
    # The storage identifier.
    name: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # The mounted device.
    device: str
    # The mount options.
    options: str
    # The mount path.
    path: str
    # The filesystem type.
    type: str
    # The path of the mount unit.
    unitfile: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class DirectoryClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'directory'}"

    def name(self, name: str) -> _name.NameClient:
        return _name.NameClient(
            self.client,
            self.path,
            name,
        )

    def get(self) -> list[GetResponseItem]:
        """
        PVE Managed Directory storages.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters) -> str:
        """
        Create a Filesystem on an unused disk. Will be mounted under '/mnt/pve/NAME'.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncDirectoryClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'directory'}"

    def name(self, name: str) -> _name.AsyncNameClient:
        return _name.AsyncNameClient(
            self.client,
            self.path,
            name,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        PVE Managed Directory storages.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters) -> str:
        """
        Create a Filesystem on an unused disk. Will be mounted under '/mnt/pve/NAME'.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
