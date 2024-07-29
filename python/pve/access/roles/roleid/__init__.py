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
    append: Optional[bool] = Field(default=None)
    privs: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    datastore_allocate: Optional[bool] = Field(alias="Datastore.Allocate", default=None)
    datastore_allocate_space: Optional[bool] = Field(
        alias="Datastore.AllocateSpace", default=None
    )
    datastore_allocate_template: Optional[bool] = Field(
        alias="Datastore.AllocateTemplate", default=None
    )
    datastore_audit: Optional[bool] = Field(alias="Datastore.Audit", default=None)
    group_allocate: Optional[bool] = Field(alias="Group.Allocate", default=None)
    permissions_modify: Optional[bool] = Field(alias="Permissions.Modify", default=None)
    pool_allocate: Optional[bool] = Field(alias="Pool.Allocate", default=None)
    pool_audit: Optional[bool] = Field(alias="Pool.Audit", default=None)
    realm_allocate: Optional[bool] = Field(alias="Realm.Allocate", default=None)
    realm_allocate_user: Optional[bool] = Field(
        alias="Realm.AllocateUser", default=None
    )
    sdn_allocate: Optional[bool] = Field(alias="SDN.Allocate", default=None)
    sdn_audit: Optional[bool] = Field(alias="SDN.Audit", default=None)
    sdn_use: Optional[bool] = Field(alias="SDN.Use", default=None)
    sys_audit: Optional[bool] = Field(alias="Sys.Audit", default=None)
    sys_console: Optional[bool] = Field(alias="Sys.Console", default=None)
    sys_incoming: Optional[bool] = Field(alias="Sys.Incoming", default=None)
    sys_modify: Optional[bool] = Field(alias="Sys.Modify", default=None)
    sys_power_mgmt: Optional[bool] = Field(alias="Sys.PowerMgmt", default=None)
    sys_syslog: Optional[bool] = Field(alias="Sys.Syslog", default=None)
    user_modify: Optional[bool] = Field(alias="User.Modify", default=None)
    vm_allocate: Optional[bool] = Field(alias="VM.Allocate", default=None)
    vm_audit: Optional[bool] = Field(alias="VM.Audit", default=None)
    vm_backup: Optional[bool] = Field(alias="VM.Backup", default=None)
    vm_clone: Optional[bool] = Field(alias="VM.Clone", default=None)
    vm_config_cdrom: Optional[bool] = Field(alias="VM.Config.CDROM", default=None)
    vm_config_cpu: Optional[bool] = Field(alias="VM.Config.CPU", default=None)
    vm_config_cloudinit: Optional[bool] = Field(
        alias="VM.Config.Cloudinit", default=None
    )
    vm_config_disk: Optional[bool] = Field(alias="VM.Config.Disk", default=None)
    vm_config_hwtype: Optional[bool] = Field(alias="VM.Config.HWType", default=None)
    vm_config_memory: Optional[bool] = Field(alias="VM.Config.Memory", default=None)
    vm_config_network: Optional[bool] = Field(alias="VM.Config.Network", default=None)
    vm_config_options: Optional[bool] = Field(alias="VM.Config.Options", default=None)
    vm_console: Optional[bool] = Field(alias="VM.Console", default=None)
    vm_migrate: Optional[bool] = Field(alias="VM.Migrate", default=None)
    vm_monitor: Optional[bool] = Field(alias="VM.Monitor", default=None)
    vm_power_mgmt: Optional[bool] = Field(alias="VM.PowerMgmt", default=None)
    vm_snapshot: Optional[bool] = Field(alias="VM.Snapshot", default=None)
    vm_snapshot_rollback: Optional[bool] = Field(
        alias="VM.Snapshot.Rollback", default=None
    )

    class Config(CommonPydanticConfig):
        pass


@dataclass
class RoleidClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, roleid: str):
        self.client = client
        self.path = f"{parent_path}/{roleid}"

    def delete(self):
        """
        Delete role.
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> GetResponseItem:
        """
        Get role configuration.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update an existing role.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncRoleidClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, roleid: str):
        self.client = client
        self.path = f"{parent_path}/{roleid}"

    async def delete(self):
        """
        Delete role.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> GetResponseItem:
        """
        Get role configuration.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update an existing role.
        """
        return await self.client.put(self.path, parameters)
