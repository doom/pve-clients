from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import flags as _flags
from . import metadata as _metadata
from . import status as _status


class GetResponseItem(BaseModel):
    pass


@dataclass
class CephClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'ceph'}"

    def metadata(self) -> _metadata.MetadataClient:
        return _metadata.MetadataClient(
            self.client,
            self.path,
        )

    def status(self) -> _status.StatusClient:
        return _status.StatusClient(
            self.client,
            self.path,
        )

    def flags(self) -> _flags.FlagsClient:
        return _flags.FlagsClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Cluster ceph index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncCephClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'ceph'}"

    def metadata(self) -> _metadata.AsyncMetadataClient:
        return _metadata.AsyncMetadataClient(
            self.client,
            self.path,
        )

    def status(self) -> _status.AsyncStatusClient:
        return _status.AsyncStatusClient(
            self.client,
            self.path,
        )

    def flags(self) -> _flags.AsyncFlagsClient:
        return _flags.AsyncFlagsClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Cluster ceph index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
