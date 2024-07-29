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
    # OS architecture type.
    arch: Optional[str] = Field(default=None)
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
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Description for the Container. Shown in the web-interface CT's summary. This is saved as comment inside the configuration file.
    description: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Allow containers access to advanced features.
    features: Optional[str] = Field(default=None)
    # Script that will be exectued during various steps in the containers lifetime.
    hookscript: Optional[str] = Field(default=None)
    # Set a host name for the container.
    hostname: Optional[str] = Field(default=None)
    # Lock/unlock the container.
    lock: Optional[str] = Field(default=None)
    # Amount of RAM for the container in MB.
    memory: Optional[int] = Field(default=None)
    # Use volume as container mount point. Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume.
    mps: dict[int, Optional[str]] = Field(default=None)
    # Sets DNS server IP address for a container. Create will automatically use the setting from the host if you neither set searchdomain nor nameserver.
    nameserver: Optional[str] = Field(default=None)
    # Specifies network interfaces for the container.
    nets: dict[int, Optional[str]] = Field(default=None)
    # Specifies whether a container will be started during system bootup.
    onboot: Optional[bool] = Field(default=None)
    # OS type. This is used to setup configuration inside the container, and corresponds to lxc setup scripts in /usr/share/lxc/config/<ostype>.common.conf. Value 'unmanaged' can be used to skip and OS specific setup.
    ostype: Optional[str] = Field(default=None)
    # Sets the protection flag of the container. This will prevent the CT or CT's disk remove/update operation.
    protection: Optional[bool] = Field(default=None)
    # Revert a pending change.
    revert: Optional[str] = Field(default=None)
    # Use volume as container root.
    rootfs: Optional[str] = Field(default=None)
    # Sets DNS search domains for a container. Create will automatically use the setting from the host if you neither set searchdomain nor nameserver.
    searchdomain: Optional[str] = Field(default=None)
    # Startup and shutdown behavior. Order is a non-negative number defining the general startup order. Shutdown in done with reverse ordering. Additionally you can set the 'up' or 'down' delay in seconds, which specifies a delay to wait before the next VM is started or stopped.
    startup: Optional[str] = Field(default=None)
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
    # Makes the container run as unprivileged user. (Should not be modified manually.)
    unprivileged: Optional[bool] = Field(default=None)
    # Reference to unused volumes. This is used internally, and should not be modified manually.
    unuseds: dict[int, Optional[str]] = Field(default=None)

    @model_serializer(mode="wrap")
    def _serialize_repeated(self, serializer):
        data = serializer(self)
        data = serialize_repeated_with_prefix(data, group="mps", prefix="mp")
        data = serialize_repeated_with_prefix(data, group="nets", prefix="net")
        data = serialize_repeated_with_prefix(data, group="unuseds", prefix="unused")
        return data

    @model_validator(mode="before")
    @classmethod
    def _extract_repeated(cls, data: Any) -> Any:
        if not isinstance(data, dict):
            return data
        data = data = extract_repeated_with_prefix(data, group="mps", prefix="mp")
        data = data = extract_repeated_with_prefix(data, group="nets", prefix="net")
        data = data = extract_repeated_with_prefix(
            data, group="unuseds", prefix="unused"
        )
        return data

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # OS architecture type.
    arch: Optional[str] = Field(default=None)
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
    # SHA1 digest of configuration file. This can be used to prevent concurrent modifications.
    digest: str
    # Allow containers access to advanced features.
    features: Optional[str] = Field(default=None)
    # Script that will be exectued during various steps in the containers lifetime.
    hookscript: Optional[str] = Field(default=None)
    # Set a host name for the container.
    hostname: Optional[str] = Field(default=None)
    # Lock/unlock the container.
    lock: Optional[str] = Field(default=None)
    # Array of lxc low-level configurations ([[key1, value1], [key2, value2] ...]).
    lxc: Optional[list[list[str]]] = Field(default=None)
    # Amount of RAM for the container in MB.
    memory: Optional[int] = Field(default=None)
    # Use volume as container mount point. Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume.
    mps: dict[int, Optional[str]] = Field(default=None)
    # Sets DNS server IP address for a container. Create will automatically use the setting from the host if you neither set searchdomain nor nameserver.
    nameserver: Optional[str] = Field(default=None)
    # Specifies network interfaces for the container.
    nets: dict[int, Optional[str]] = Field(default=None)
    # Specifies whether a container will be started during system bootup.
    onboot: Optional[bool] = Field(default=None)
    # OS type. This is used to setup configuration inside the container, and corresponds to lxc setup scripts in /usr/share/lxc/config/<ostype>.common.conf. Value 'unmanaged' can be used to skip and OS specific setup.
    ostype: Optional[str] = Field(default=None)
    # Sets the protection flag of the container. This will prevent the CT or CT's disk remove/update operation.
    protection: Optional[bool] = Field(default=None)
    # Use volume as container root.
    rootfs: Optional[str] = Field(default=None)
    # Sets DNS search domains for a container. Create will automatically use the setting from the host if you neither set searchdomain nor nameserver.
    searchdomain: Optional[str] = Field(default=None)
    # Startup and shutdown behavior. Order is a non-negative number defining the general startup order. Shutdown in done with reverse ordering. Additionally you can set the 'up' or 'down' delay in seconds, which specifies a delay to wait before the next VM is started or stopped.
    startup: Optional[str] = Field(default=None)
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
    # Makes the container run as unprivileged user. (Should not be modified manually.)
    unprivileged: Optional[bool] = Field(default=None)
    # Reference to unused volumes. This is used internally, and should not be modified manually.
    unuseds: dict[int, Optional[str]] = Field(default=None)

    @model_serializer(mode="wrap")
    def _serialize_repeated(self, serializer):
        data = serializer(self)
        data = serialize_repeated_with_prefix(data, group="mps", prefix="mp")
        data = serialize_repeated_with_prefix(data, group="nets", prefix="net")
        data = serialize_repeated_with_prefix(data, group="unuseds", prefix="unused")
        return data

    @model_validator(mode="before")
    @classmethod
    def _extract_repeated(cls, data: Any) -> Any:
        if not isinstance(data, dict):
            return data
        data = data = extract_repeated_with_prefix(data, group="mps", prefix="mp")
        data = data = extract_repeated_with_prefix(data, group="nets", prefix="net")
        data = data = extract_repeated_with_prefix(
            data, group="unuseds", prefix="unused"
        )
        return data

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Get current values (instead of pending values).
    current: Optional[bool] = Field(default=None)
    # Fetch config values from given snapshot.
    snapshot: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ConfigClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'config'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get container configuration.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Set container options.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncConfigClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'config'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get container configuration.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Set container options.
        """
        return await self.client.put(self.path, parameters)
