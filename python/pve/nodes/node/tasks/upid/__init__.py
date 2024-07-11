from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import log as _log
from . import status as _status


class GetResponseItem(BaseModel):
    pass


@dataclass
class UpidClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, upid: str):
        self.client = client
        self.path = f"{parent_path}/{upid}"

    def log(self) -> _log.LogClient:
        return _log.LogClient(
            self.client,
            self.path,
        )

    def status(self) -> _status.StatusClient:
        return _status.StatusClient(
            self.client,
            self.path,
        )

    def delete(self):
        """
        Stop a task.
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """ """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncUpidClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, upid: str):
        self.client = client
        self.path = f"{parent_path}/{upid}"

    def log(self) -> _log.AsyncLogClient:
        return _log.AsyncLogClient(
            self.client,
            self.path,
        )

    def status(self) -> _status.AsyncStatusClient:
        return _status.AsyncStatusClient(
            self.client,
            self.path,
        )

    async def delete(self):
        """
        Stop a task.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """ """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
