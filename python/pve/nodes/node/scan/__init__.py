from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import cifs as _cifs
from . import glusterfs as _glusterfs
from . import iscsi as _iscsi
from . import lvm as _lvm
from . import lvmthin as _lvmthin
from . import nfs as _nfs
from . import pbs as _pbs
from . import zfs as _zfs


class GetResponseItem(BaseModel):
    method: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ScanClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'scan'}"

    def nfs(self) -> _nfs.NfsClient:
        return _nfs.NfsClient(
            self.client,
            self.path,
        )

    def cifs(self) -> _cifs.CifsClient:
        return _cifs.CifsClient(
            self.client,
            self.path,
        )

    def pbs(self) -> _pbs.PbsClient:
        return _pbs.PbsClient(
            self.client,
            self.path,
        )

    def glusterfs(self) -> _glusterfs.GlusterfsClient:
        return _glusterfs.GlusterfsClient(
            self.client,
            self.path,
        )

    def iscsi(self) -> _iscsi.IscsiClient:
        return _iscsi.IscsiClient(
            self.client,
            self.path,
        )

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

    def zfs(self) -> _zfs.ZfsClient:
        return _zfs.ZfsClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Index of available scan methods
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncScanClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'scan'}"

    def nfs(self) -> _nfs.AsyncNfsClient:
        return _nfs.AsyncNfsClient(
            self.client,
            self.path,
        )

    def cifs(self) -> _cifs.AsyncCifsClient:
        return _cifs.AsyncCifsClient(
            self.client,
            self.path,
        )

    def pbs(self) -> _pbs.AsyncPbsClient:
        return _pbs.AsyncPbsClient(
            self.client,
            self.path,
        )

    def glusterfs(self) -> _glusterfs.AsyncGlusterfsClient:
        return _glusterfs.AsyncGlusterfsClient(
            self.client,
            self.path,
        )

    def iscsi(self) -> _iscsi.AsyncIscsiClient:
        return _iscsi.AsyncIscsiClient(
            self.client,
            self.path,
        )

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

    def zfs(self) -> _zfs.AsyncZfsClient:
        return _zfs.AsyncZfsClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Index of available scan methods
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
