from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import vmid as _vmid


class PostParameters(BaseModel):
    # OS architecture type.
    arch: Optional[str] = Field(default=None)
    # Override I/O bandwidth limit (in KiB/s).
    bwlimit: Optional[float] = Field(default=None)
    # Console mode. By default, the console command tries to open a connection to one of the available tty devices. By setting cmode to 'console' it tries to attach to /dev/console instead. If you set cmode to 'shell', it simply invokes a shell inside the container (no login).
    cmode: Optional[str] = Field(default=None)
    # Attach a console device (/dev/console) to the container.
    console: Optional[bool] = Field(default=None)
    # The number of cores assigned to the container. A container can use all available cores by default.
    cores: Optional[int] = Field(default=None)
    # Limit of CPU usage.  NOTE: If the computer has 2 CPUs, it has a total of '2' CPU time. Value '0' indicates no CPU limit.
    cpulimit: Optional[float] = Field(default=None)
    # CPU weight for a container, will be clamped to [1, 10000] in cgroup v2.
    cpuunits: Optional[int] = Field(default=None)
    # Try to be more verbose. For now this only enables debug log-level on start.
    debug: Optional[bool] = Field(default=None)
    # Description for the Container. Shown in the web-interface CT's summary. This is saved as comment inside the configuration file.
    description: Optional[str] = Field(default=None)
    # Allow containers access to advanced features.
    features: Optional[str] = Field(default=None)
    # Allow to overwrite existing container.
    force: Optional[bool] = Field(default=None)
    # Script that will be exectued during various steps in the containers lifetime.
    hookscript: Optional[str] = Field(default=None)
    # Set a host name for the container.
    hostname: Optional[str] = Field(default=None)
    # Ignore errors when extracting the template.
    ignore_unpack_errors: Optional[bool] = Field(
        alias="ignore-unpack-errors", default=None
    )
    # Lock/unlock the container.
    lock: Optional[str] = Field(default=None)
    # Amount of RAM for the container in MB.
    memory: Optional[int] = Field(default=None)
    # Use volume as container mount point. Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume.
    mps: dict[int, Optional[str]] = Field(alias="mp[n]", default=None)
    # Sets DNS server IP address for a container. Create will automatically use the setting from the host if you neither set searchdomain nor nameserver.
    nameserver: Optional[str] = Field(default=None)
    # Specifies network interfaces for the container.
    nets: dict[int, Optional[str]] = Field(alias="net[n]", default=None)
    # Specifies whether a container will be started during system bootup.
    onboot: Optional[bool] = Field(default=None)
    # The OS template or backup file.
    ostemplate: str
    # OS type. This is used to setup configuration inside the container, and corresponds to lxc setup scripts in /usr/share/lxc/config/<ostype>.common.conf. Value 'unmanaged' can be used to skip and OS specific setup.
    ostype: Optional[str] = Field(default=None)
    # Sets root password inside container.
    password: Optional[str] = Field(default=None)
    # Add the VM to the specified pool.
    pool: Optional[str] = Field(default=None)
    # Sets the protection flag of the container. This will prevent the CT or CT's disk remove/update operation.
    protection: Optional[bool] = Field(default=None)
    # Mark this as restore task.
    restore: Optional[bool] = Field(default=None)
    # Use volume as container root.
    rootfs: Optional[str] = Field(default=None)
    # Sets DNS search domains for a container. Create will automatically use the setting from the host if you neither set searchdomain nor nameserver.
    searchdomain: Optional[str] = Field(default=None)
    # Setup public SSH keys (one key per line, OpenSSH format).
    ssh_public_keys: Optional[str] = Field(alias="ssh-public-keys", default=None)
    # Start the CT after its creation finished successfully.
    start: Optional[bool] = Field(default=None)
    # Startup and shutdown behavior. Order is a non-negative number defining the general startup order. Shutdown in done with reverse ordering. Additionally you can set the 'up' or 'down' delay in seconds, which specifies a delay to wait before the next VM is started or stopped.
    startup: Optional[str] = Field(default=None)
    # Default Storage.
    storage: Optional[str] = Field(default=None)
    # Amount of SWAP for the container in MB.
    swap: Optional[int] = Field(default=None)
    # Tags of the Container. This is only meta information.
    tags: Optional[str] = Field(default=None)
    # Enable/disable Template.
    template: Optional[bool] = Field(default=None)
    # Time zone to use in the container. If option isn't set, then nothing will be done. Can be set to 'host' to match the host time zone, or an arbitrary time zone option from /usr/share/zoneinfo/zone.tab
    timezone: Optional[str] = Field(default=None)
    # Specify the number of tty available to the container
    tty: Optional[int] = Field(default=None)
    # Assign a unique random ethernet address.
    unique: Optional[bool] = Field(default=None)
    # Makes the container run as unprivileged user. (Should not be modified manually.)
    unprivileged: Optional[bool] = Field(default=None)
    # Reference to unused volumes. This is used internally, and should not be modified manually.
    unuseds: dict[int, Optional[str]] = Field(alias="unused[n]", default=None)
    # The (unique) ID of the VM.
    vmid: int

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Maximum usable CPUs.
    cpus: Optional[float] = Field(default=None)
    # The current config lock, if any.
    lock: Optional[str] = Field(default=None)
    # Root disk size in bytes.
    maxdisk: Optional[int] = Field(default=None)
    # Maximum memory in bytes.
    maxmem: Optional[int] = Field(default=None)
    # Maximum SWAP memory in bytes.
    maxswap: Optional[int] = Field(default=None)
    # Container name.
    name: Optional[str] = Field(default=None)
    # LXC Container status.
    status: str
    # The current configured tags, if any.
    tags: Optional[str] = Field(default=None)
    # Uptime.
    uptime: Optional[int] = Field(default=None)
    # The (unique) ID of the VM.
    vmid: int

    class Config(CommonPydanticConfig):
        pass


@dataclass
class LxcClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'lxc'}"

    def vmid(self, vmid: str) -> _vmid.VmidClient:
        return _vmid.VmidClient(
            self.client,
            self.path,
            vmid,
        )

    def get(self) -> list[GetResponseItem]:
        """
        LXC container index (per node).
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters) -> str:
        """
        Create or restore a container.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncLxcClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'lxc'}"

    def vmid(self, vmid: str) -> _vmid.AsyncVmidClient:
        return _vmid.AsyncVmidClient(
            self.client,
            self.path,
            vmid,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        LXC container index (per node).
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters) -> str:
        """
        Create or restore a container.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
