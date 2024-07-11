from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # Override I/O bandwidth limit (in KiB/s).
    bwlimit: Optional[int] = Field(default=None)
    # Allow to migrate VMs which use local devices. Only root may use this option.
    force: Optional[bool] = Field(default=None)
    # CIDR of the (sub) network that is used for migration.
    migration_network: Optional[str] = Field(default=None)
    # Migration traffic is encrypted using an SSH tunnel by default. On secure, completely private networks this can be disabled to increase performance.
    migration_type: Optional[str] = Field(default=None)
    # Use online/live migration if VM is running. Ignored if VM is stopped.
    online: Optional[bool] = Field(default=None)
    # Target node.
    target: str
    # Mapping from source to target storages. Providing only a single storage ID maps all source storages to that storage. Providing the special value '1' will map each source storage to itself.
    targetstorage: Optional[str] = Field(default=None)
    # Enable live storage migration for local disk
    with_local_disks: Optional[bool] = Field(alias="with-local-disks", default=None)

    class Config(CommonPydanticConfig):
        pass


class NotAllowedNodes(BaseModel):
    """
    List not allowed nodes with additional informations, only passed if VM is offline
    """

    pass


class GetResponseItem(BaseModel):
    # List nodes allowed for offline migration, only passed if VM is offline
    allowed_nodes: Optional[list[dict[str, Any]]] = Field(default=None)
    # List local disks including CD-Rom, unsused and not referenced disks
    local_disks: list[dict[str, Any]]
    # List local resources e.g. pci, usb
    local_resources: list[dict[str, Any]]
    # List not allowed nodes with additional informations, only passed if VM is offline
    not_allowed_nodes: Optional[NotAllowedNodes] = Field(default=None)
    running: bool

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Target node.
    target: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MigrateClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'migrate'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get preconditions for migration.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)

    def post(self, parameters: PostParameters) -> str:
        """
        Migrate virtual machine. Creates a new migration task.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncMigrateClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'migrate'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get preconditions for migration.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)

    async def post(self, parameters: PostParameters) -> str:
        """
        Migrate virtual machine. Creates a new migration task.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
