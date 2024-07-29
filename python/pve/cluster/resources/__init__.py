from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetResponseItem(BaseModel):
    # The cgroup mode the node operates under (when type == node).
    cgroup_mode: Optional[int] = Field(alias="cgroup-mode", default=None)
    # Allowed storage content types (when type == storage).
    content: Optional[str] = Field(default=None)
    # CPU utilization (when type in node,qemu,lxc).
    cpu: Optional[float] = Field(default=None)
    # Used disk space in bytes (when type in storage), used root image spave for VMs (type in qemu,lxc).
    disk: Optional[int] = Field(default=None)
    # HA service status (for HA managed VMs).
    hastate: Optional[str] = Field(default=None)
    # Resource id.
    id: str
    # Support level (when type == node).
    level: Optional[str] = Field(default=None)
    # Number of available CPUs (when type in node,qemu,lxc).
    maxcpu: Optional[float] = Field(default=None)
    # Storage size in bytes (when type in storage), root image size for VMs (type in qemu,lxc).
    maxdisk: Optional[int] = Field(default=None)
    # Number of available memory in bytes (when type in node,qemu,lxc).
    maxmem: Optional[int] = Field(default=None)
    # Used memory in bytes (when type in node,qemu,lxc).
    mem: Optional[int] = Field(default=None)
    # Name of the resource.
    name: Optional[str] = Field(default=None)
    # The cluster node name (when type in node,storage,qemu,lxc).
    node: Optional[str] = Field(default=None)
    # More specific type, if available.
    plugintype: Optional[str] = Field(default=None)
    # The pool name (when type in pool,qemu,lxc).
    pool: Optional[str] = Field(default=None)
    # Resource type dependent status.
    status: Optional[str] = Field(default=None)
    # The storage identifier (when type == storage).
    storage: Optional[str] = Field(default=None)
    # Resource type.
    type: str
    # Node uptime in seconds (when type in node,qemu,lxc).
    uptime: Optional[int] = Field(default=None)
    # The numerical vmid (when type in qemu,lxc).
    vmid: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ResourcesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'resources'}"

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Resources index (cluster wide).
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncResourcesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'resources'}"

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Resources index (cluster wide).
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
