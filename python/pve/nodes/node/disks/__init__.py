from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import directory as _directory
from . import initgpt as _initgpt
from . import list_ as _list_
from . import lvm as _lvm
from . import lvmthin as _lvmthin
from . import smart as _smart
from . import wipedisk as _wipedisk
from . import zfs as _zfs


class GetResponseItem(BaseModel):
    pass


@dataclass
class DisksClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'disks'}"

    def lvm(self) -> _lvm.LvmClient:
        return _lvm.LvmClient(
            self.client,
            self.path,
        )

    def lvmthin(self) -> _lvmthin.LvmthinClient:
        return _lvmthin.LvmthinClient(
            self.client,
            self.path,
        )

    def directory(self) -> _directory.DirectoryClient:
        return _directory.DirectoryClient(
            self.client,
            self.path,
        )

    def zfs(self) -> _zfs.ZfsClient:
        return _zfs.ZfsClient(
            self.client,
            self.path,
        )

    def list_(self) -> _list_.ListClient:
        return _list_.ListClient(
            self.client,
            self.path,
        )

    def smart(self) -> _smart.SmartClient:
        return _smart.SmartClient(
            self.client,
            self.path,
        )

    def initgpt(self) -> _initgpt.InitgptClient:
        return _initgpt.InitgptClient(
            self.client,
            self.path,
        )

    def wipedisk(self) -> _wipedisk.WipediskClient:
        return _wipedisk.WipediskClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Node index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncDisksClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'disks'}"

    def lvm(self) -> _lvm.AsyncLvmClient:
        return _lvm.AsyncLvmClient(
            self.client,
            self.path,
        )

    def lvmthin(self) -> _lvmthin.AsyncLvmthinClient:
        return _lvmthin.AsyncLvmthinClient(
            self.client,
            self.path,
        )

    def directory(self) -> _directory.AsyncDirectoryClient:
        return _directory.AsyncDirectoryClient(
            self.client,
            self.path,
        )

    def zfs(self) -> _zfs.AsyncZfsClient:
        return _zfs.AsyncZfsClient(
            self.client,
            self.path,
        )

    def list_(self) -> _list_.AsyncListClient:
        return _list_.AsyncListClient(
            self.client,
            self.path,
        )

    def smart(self) -> _smart.AsyncSmartClient:
        return _smart.AsyncSmartClient(
            self.client,
            self.path,
        )

    def initgpt(self) -> _initgpt.AsyncInitgptClient:
        return _initgpt.AsyncInitgptClient(
            self.client,
            self.path,
        )

    def wipedisk(self) -> _wipedisk.AsyncWipediskClient:
        return _wipedisk.AsyncWipediskClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Node index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
