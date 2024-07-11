from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import download as _download
from . import list_ as _list_


@dataclass
class FileRestoreClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'file-restore'}"

    def list_(self) -> _list_.ListClient:
        return _list_.ListClient(
            self.client,
            self.path,
        )

    def download(self) -> _download.DownloadClient:
        return _download.DownloadClient(
            self.client,
            self.path,
        )


@dataclass
class AsyncFileRestoreClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'file-restore'}"

    def list_(self) -> _list_.AsyncListClient:
        return _list_.AsyncListClient(
            self.client,
            self.path,
        )

    def download(self) -> _download.AsyncDownloadClient:
        return _download.AsyncDownloadClient(
            self.client,
            self.path,
        )
