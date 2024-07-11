from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # If set, does not write anything.
    dry_run: Optional[bool] = Field(alias="dry-run", default=None)
    # Enable newly synced users immediately.
    enable_new: Optional[bool] = Field(alias="enable-new", default=None)
    # DEPRECATED: use 'remove-vanished' instead. If set, uses the LDAP Directory as source of truth, deleting users or groups not returned from the sync and removing all locally modified properties of synced users. If not set, only syncs information which is present in the synced data, and does not delete or modify anything else.
    full: Optional[bool] = Field(default=None)
    # DEPRECATED: use 'remove-vanished' instead. Remove ACLs for users or groups which were removed from the config during a sync.
    purge: Optional[bool] = Field(default=None)
    # A semicolon-seperated list of things to remove when they or the user vanishes during a sync. The following values are possible: 'entry' removes the user/group when not returned from the sync. 'properties' removes the set properties on existing user/group that do not appear in the source (even custom ones). 'acl' removes acls when the user/group is not returned from the sync. Instead of a list it also can be 'none' (the default).
    remove_vanished: Optional[str] = Field(alias="remove-vanished", default=None)
    # Select what to sync.
    scope: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class SyncClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'sync'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Syncs users and/or groups from the configured LDAP to user.cfg. NOTE: Synced groups will have the name 'name-$realm', so make sure those groups do not exist to prevent overwriting.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncSyncClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'sync'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Syncs users and/or groups from the configured LDAP to user.cfg. NOTE: Synced groups will have the name 'name-$realm', so make sure those groups do not exist to prevent overwriting.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
