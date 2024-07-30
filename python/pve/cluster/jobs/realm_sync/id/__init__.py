from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PutParameters(BaseModel):
    # Description for the Job.
    comment: Optional[str] = Field(default=None)
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Enable newly synced users immediately.
    enable_new: Optional[bool] = Field(alias="enable-new", default=None)
    # Determines if the job is enabled.
    enabled: Optional[bool] = Field(default=None)
    # A semicolon-seperated list of things to remove when they or the user vanishes during a sync. The following values are possible: 'entry' removes the user/group when not returned from the sync. 'properties' removes the set properties on existing user/group that do not appear in the source (even custom ones). 'acl' removes acls when the user/group is not returned from the sync. Instead of a list it also can be 'none' (the default).
    remove_vanished: Optional[str] = Field(alias="remove-vanished", default=None)
    # Backup schedule. The format is a subset of `systemd` calendar events.
    schedule: str
    # Select what to sync.
    scope: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # Description for the Job.
    comment: Optional[str] = Field(default=None)
    # Enable newly synced users immediately.
    enable_new: Optional[bool] = Field(alias="enable-new", default=None)
    # Determines if the job is enabled.
    enabled: Optional[bool] = Field(default=None)
    # Authentication domain ID
    realm: Optional[str] = Field(default=None)
    # A semicolon-seperated list of things to remove when they or the user vanishes during a sync. The following values are possible: 'entry' removes the user/group when not returned from the sync. 'properties' removes the set properties on existing user/group that do not appear in the source (even custom ones). 'acl' removes acls when the user/group is not returned from the sync. Instead of a list it also can be 'none' (the default).
    remove_vanished: Optional[str] = Field(alias="remove-vanished", default=None)
    # Backup schedule. The format is a subset of `systemd` calendar events.
    schedule: str
    # Select what to sync.
    scope: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


@dataclass
class IdClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, id: str):
        self.client = client
        self.path = f"{parent_path}/{id}"

    def delete(self):
        """
        Delete realm-sync job definition.
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> GetResponseItem:
        """
        Read realm-sync job definition.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def post(self, parameters: PostParameters):
        """
        Create new realm-sync job.
        """
        return self.client.post(self.path, parameters)

    def put(self, parameters: PutParameters):
        """
        Update realm-sync job definition.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncIdClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, id: str):
        self.client = client
        self.path = f"{parent_path}/{id}"

    async def delete(self):
        """
        Delete realm-sync job definition.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> GetResponseItem:
        """
        Read realm-sync job definition.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def post(self, parameters: PostParameters):
        """
        Create new realm-sync job.
        """
        return await self.client.post(self.path, parameters)

    async def put(self, parameters: PutParameters):
        """
        Update realm-sync job definition.
        """
        return await self.client.put(self.path, parameters)
