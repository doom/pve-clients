from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import content as _content
from . import download_url as _download_url
from . import file_restore as _file_restore
from . import prunebackups as _prunebackups
from . import rrd as _rrd
from . import rrddata as _rrddata
from . import status as _status
from . import upload as _upload


class GetResponseItem(BaseModel):
    subdir: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class StorageClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, storage: str):
        self.client = client
        self.path = f"{parent_path}/{storage}"

    def prunebackups(self) -> _prunebackups.PrunebackupsClient:
        return _prunebackups.PrunebackupsClient(
            self.client,
            self.path,
        )

    def content(self) -> _content.ContentClient:
        return _content.ContentClient(
            self.client,
            self.path,
        )

    def file_restore(self) -> _file_restore.FileRestoreClient:
        return _file_restore.FileRestoreClient(
            self.client,
            self.path,
        )

    def status(self) -> _status.StatusClient:
        return _status.StatusClient(
            self.client,
            self.path,
        )

    def rrd(self) -> _rrd.RrdClient:
        return _rrd.RrdClient(
            self.client,
            self.path,
        )

    def rrddata(self) -> _rrddata.RrddataClient:
        return _rrddata.RrddataClient(
            self.client,
            self.path,
        )

    def upload(self) -> _upload.UploadClient:
        return _upload.UploadClient(
            self.client,
            self.path,
        )

    def download_url(self) -> _download_url.DownloadUrlClient:
        return _download_url.DownloadUrlClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """ """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncStorageClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, storage: str):
        self.client = client
        self.path = f"{parent_path}/{storage}"

    def prunebackups(self) -> _prunebackups.AsyncPrunebackupsClient:
        return _prunebackups.AsyncPrunebackupsClient(
            self.client,
            self.path,
        )

    def content(self) -> _content.AsyncContentClient:
        return _content.AsyncContentClient(
            self.client,
            self.path,
        )

    def file_restore(self) -> _file_restore.AsyncFileRestoreClient:
        return _file_restore.AsyncFileRestoreClient(
            self.client,
            self.path,
        )

    def status(self) -> _status.AsyncStatusClient:
        return _status.AsyncStatusClient(
            self.client,
            self.path,
        )

    def rrd(self) -> _rrd.AsyncRrdClient:
        return _rrd.AsyncRrdClient(
            self.client,
            self.path,
        )

    def rrddata(self) -> _rrddata.AsyncRrddataClient:
        return _rrddata.AsyncRrddataClient(
            self.client,
            self.path,
        )

    def upload(self) -> _upload.AsyncUploadClient:
        return _upload.AsyncUploadClient(
            self.client,
            self.path,
        )

    def download_url(self) -> _download_url.AsyncDownloadUrlClient:
        return _download_url.AsyncDownloadUrlClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """ """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
