from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import log as _log
from . import schedule_now as _schedule_now
from . import status as _status


class GetResponseItem(BaseModel):
    pass


@dataclass
class IdClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, id: str):
        self.client = client
        self.path = f"{parent_path}/{id}"

    def status(self) -> _status.StatusClient:
        return _status.StatusClient(
            self.client,
            self.path,
        )

    def log(self) -> _log.LogClient:
        return _log.LogClient(
            self.client,
            self.path,
        )

    def schedule_now(self) -> _schedule_now.ScheduleNowClient:
        return _schedule_now.ScheduleNowClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncIdClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, id: str):
        self.client = client
        self.path = f"{parent_path}/{id}"

    def status(self) -> _status.AsyncStatusClient:
        return _status.AsyncStatusClient(
            self.client,
            self.path,
        )

    def log(self) -> _log.AsyncLogClient:
        return _log.AsyncLogClient(
            self.client,
            self.path,
        )

    def schedule_now(self) -> _schedule_now.AsyncScheduleNowClient:
        return _schedule_now.AsyncScheduleNowClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
