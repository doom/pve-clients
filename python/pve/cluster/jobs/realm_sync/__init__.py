from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import id as _id


class GetResponseItem(BaseModel):
    # A comment for the job.
    comment: Optional[str] = Field(default=None)
    # If the job is enabled or not.
    enabled: bool
    # The ID of the entry.
    id: str
    # Last execution time of the job in seconds since the beginning of the UNIX epoch
    last_run: Optional[int] = Field(alias="last-run", default=None)
    # Next planned execution time of the job in seconds since the beginning of the UNIX epoch.
    next_run: Optional[int] = Field(alias="next-run", default=None)
    # Authentication domain ID
    realm: str
    # A semicolon-seperated list of things to remove when they or the user vanishes during a sync. The following values are possible: 'entry' removes the user/group when not returned from the sync. 'properties' removes the set properties on existing user/group that do not appear in the source (even custom ones). 'acl' removes acls when the user/group is not returned from the sync. Instead of a list it also can be 'none' (the default).
    remove_vanished: Optional[str] = Field(alias="remove-vanished", default=None)
    # The configured sync schedule.
    schedule: str
    # Select what to sync.
    scope: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class RealmSyncClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'realm-sync'}"

    def id(self, id: str) -> _id.IdClient:
        return _id.IdClient(
            self.client,
            self.path,
            id,
        )

    def get(self) -> list[GetResponseItem]:
        """
        List configured realm-sync-jobs.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncRealmSyncClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'realm-sync'}"

    def id(self, id: str) -> _id.AsyncIdClient:
        return _id.AsyncIdClient(
            self.client,
            self.path,
            id,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        List configured realm-sync-jobs.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
