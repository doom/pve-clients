from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import realm_sync as _realm_sync
from . import schedule_analyze as _schedule_analyze


class GetResponseItem(BaseModel):
    # API sub-directory endpoint
    subdir: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class JobsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'jobs'}"

    def realm_sync(self) -> _realm_sync.RealmSyncClient:
        return _realm_sync.RealmSyncClient(
            self.client,
            self.path,
        )

    def schedule_analyze(self) -> _schedule_analyze.ScheduleAnalyzeClient:
        return _schedule_analyze.ScheduleAnalyzeClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Index for jobs related endpoints.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncJobsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'jobs'}"

    def realm_sync(self) -> _realm_sync.AsyncRealmSyncClient:
        return _realm_sync.AsyncRealmSyncClient(
            self.client,
            self.path,
        )

    def schedule_analyze(self) -> _schedule_analyze.AsyncScheduleAnalyzeClient:
        return _schedule_analyze.AsyncScheduleAnalyzeClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Index for jobs related endpoints.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
