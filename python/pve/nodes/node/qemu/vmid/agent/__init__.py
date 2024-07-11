from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import exec as _exec
from . import exec_status as _exec_status
from . import file_read as _file_read
from . import file_write as _file_write
from . import fsfreeze_freeze as _fsfreeze_freeze
from . import fsfreeze_status as _fsfreeze_status
from . import fsfreeze_thaw as _fsfreeze_thaw
from . import fstrim as _fstrim
from . import get_fsinfo as _get_fsinfo
from . import get_host_name as _get_host_name
from . import get_memory_block_info as _get_memory_block_info
from . import get_memory_blocks as _get_memory_blocks
from . import get_osinfo as _get_osinfo
from . import get_time as _get_time
from . import get_timezone as _get_timezone
from . import get_users as _get_users
from . import get_vcpus as _get_vcpus
from . import info as _info
from . import network_get_interfaces as _network_get_interfaces
from . import ping as _ping
from . import set_user_password as _set_user_password
from . import shutdown as _shutdown
from . import suspend_disk as _suspend_disk
from . import suspend_hybrid as _suspend_hybrid
from . import suspend_ram as _suspend_ram


class PostResponseItem(BaseModel):
    """
    Returns an object with a single `result` property.
    """

    pass


class PostParameters(BaseModel):
    # The QGA command.
    command: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


@dataclass
class AgentClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'agent'}"

    def fsfreeze_freeze(self) -> _fsfreeze_freeze.FsfreezeFreezeClient:
        return _fsfreeze_freeze.FsfreezeFreezeClient(
            self.client,
            self.path,
        )

    def fsfreeze_status(self) -> _fsfreeze_status.FsfreezeStatusClient:
        return _fsfreeze_status.FsfreezeStatusClient(
            self.client,
            self.path,
        )

    def fsfreeze_thaw(self) -> _fsfreeze_thaw.FsfreezeThawClient:
        return _fsfreeze_thaw.FsfreezeThawClient(
            self.client,
            self.path,
        )

    def fstrim(self) -> _fstrim.FstrimClient:
        return _fstrim.FstrimClient(
            self.client,
            self.path,
        )

    def get_fsinfo(self) -> _get_fsinfo.GetFsinfoClient:
        return _get_fsinfo.GetFsinfoClient(
            self.client,
            self.path,
        )

    def get_host_name(self) -> _get_host_name.GetHostNameClient:
        return _get_host_name.GetHostNameClient(
            self.client,
            self.path,
        )

    def get_memory_block_info(self) -> _get_memory_block_info.GetMemoryBlockInfoClient:
        return _get_memory_block_info.GetMemoryBlockInfoClient(
            self.client,
            self.path,
        )

    def get_memory_blocks(self) -> _get_memory_blocks.GetMemoryBlocksClient:
        return _get_memory_blocks.GetMemoryBlocksClient(
            self.client,
            self.path,
        )

    def get_osinfo(self) -> _get_osinfo.GetOsinfoClient:
        return _get_osinfo.GetOsinfoClient(
            self.client,
            self.path,
        )

    def get_time(self) -> _get_time.GetTimeClient:
        return _get_time.GetTimeClient(
            self.client,
            self.path,
        )

    def get_timezone(self) -> _get_timezone.GetTimezoneClient:
        return _get_timezone.GetTimezoneClient(
            self.client,
            self.path,
        )

    def get_users(self) -> _get_users.GetUsersClient:
        return _get_users.GetUsersClient(
            self.client,
            self.path,
        )

    def get_vcpus(self) -> _get_vcpus.GetVcpusClient:
        return _get_vcpus.GetVcpusClient(
            self.client,
            self.path,
        )

    def info(self) -> _info.InfoClient:
        return _info.InfoClient(
            self.client,
            self.path,
        )

    def network_get_interfaces(
        self,
    ) -> _network_get_interfaces.NetworkGetInterfacesClient:
        return _network_get_interfaces.NetworkGetInterfacesClient(
            self.client,
            self.path,
        )

    def ping(self) -> _ping.PingClient:
        return _ping.PingClient(
            self.client,
            self.path,
        )

    def shutdown(self) -> _shutdown.ShutdownClient:
        return _shutdown.ShutdownClient(
            self.client,
            self.path,
        )

    def suspend_disk(self) -> _suspend_disk.SuspendDiskClient:
        return _suspend_disk.SuspendDiskClient(
            self.client,
            self.path,
        )

    def suspend_hybrid(self) -> _suspend_hybrid.SuspendHybridClient:
        return _suspend_hybrid.SuspendHybridClient(
            self.client,
            self.path,
        )

    def suspend_ram(self) -> _suspend_ram.SuspendRamClient:
        return _suspend_ram.SuspendRamClient(
            self.client,
            self.path,
        )

    def set_user_password(self) -> _set_user_password.SetUserPasswordClient:
        return _set_user_password.SetUserPasswordClient(
            self.client,
            self.path,
        )

    def exec(self) -> _exec.ExecClient:
        return _exec.ExecClient(
            self.client,
            self.path,
        )

    def exec_status(self) -> _exec_status.ExecStatusClient:
        return _exec_status.ExecStatusClient(
            self.client,
            self.path,
        )

    def file_read(self) -> _file_read.FileReadClient:
        return _file_read.FileReadClient(
            self.client,
            self.path,
        )

    def file_write(self) -> _file_write.FileWriteClient:
        return _file_write.FileWriteClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        QEMU Guest Agent command index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Execute QEMU Guest Agent commands.
        """
        return self.client.post(self.path, parameters, parse_as=PostResponseItem)


@dataclass
class AsyncAgentClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'agent'}"

    def fsfreeze_freeze(self) -> _fsfreeze_freeze.AsyncFsfreezeFreezeClient:
        return _fsfreeze_freeze.AsyncFsfreezeFreezeClient(
            self.client,
            self.path,
        )

    def fsfreeze_status(self) -> _fsfreeze_status.AsyncFsfreezeStatusClient:
        return _fsfreeze_status.AsyncFsfreezeStatusClient(
            self.client,
            self.path,
        )

    def fsfreeze_thaw(self) -> _fsfreeze_thaw.AsyncFsfreezeThawClient:
        return _fsfreeze_thaw.AsyncFsfreezeThawClient(
            self.client,
            self.path,
        )

    def fstrim(self) -> _fstrim.AsyncFstrimClient:
        return _fstrim.AsyncFstrimClient(
            self.client,
            self.path,
        )

    def get_fsinfo(self) -> _get_fsinfo.AsyncGetFsinfoClient:
        return _get_fsinfo.AsyncGetFsinfoClient(
            self.client,
            self.path,
        )

    def get_host_name(self) -> _get_host_name.AsyncGetHostNameClient:
        return _get_host_name.AsyncGetHostNameClient(
            self.client,
            self.path,
        )

    def get_memory_block_info(
        self,
    ) -> _get_memory_block_info.AsyncGetMemoryBlockInfoClient:
        return _get_memory_block_info.AsyncGetMemoryBlockInfoClient(
            self.client,
            self.path,
        )

    def get_memory_blocks(self) -> _get_memory_blocks.AsyncGetMemoryBlocksClient:
        return _get_memory_blocks.AsyncGetMemoryBlocksClient(
            self.client,
            self.path,
        )

    def get_osinfo(self) -> _get_osinfo.AsyncGetOsinfoClient:
        return _get_osinfo.AsyncGetOsinfoClient(
            self.client,
            self.path,
        )

    def get_time(self) -> _get_time.AsyncGetTimeClient:
        return _get_time.AsyncGetTimeClient(
            self.client,
            self.path,
        )

    def get_timezone(self) -> _get_timezone.AsyncGetTimezoneClient:
        return _get_timezone.AsyncGetTimezoneClient(
            self.client,
            self.path,
        )

    def get_users(self) -> _get_users.AsyncGetUsersClient:
        return _get_users.AsyncGetUsersClient(
            self.client,
            self.path,
        )

    def get_vcpus(self) -> _get_vcpus.AsyncGetVcpusClient:
        return _get_vcpus.AsyncGetVcpusClient(
            self.client,
            self.path,
        )

    def info(self) -> _info.AsyncInfoClient:
        return _info.AsyncInfoClient(
            self.client,
            self.path,
        )

    def network_get_interfaces(
        self,
    ) -> _network_get_interfaces.AsyncNetworkGetInterfacesClient:
        return _network_get_interfaces.AsyncNetworkGetInterfacesClient(
            self.client,
            self.path,
        )

    def ping(self) -> _ping.AsyncPingClient:
        return _ping.AsyncPingClient(
            self.client,
            self.path,
        )

    def shutdown(self) -> _shutdown.AsyncShutdownClient:
        return _shutdown.AsyncShutdownClient(
            self.client,
            self.path,
        )

    def suspend_disk(self) -> _suspend_disk.AsyncSuspendDiskClient:
        return _suspend_disk.AsyncSuspendDiskClient(
            self.client,
            self.path,
        )

    def suspend_hybrid(self) -> _suspend_hybrid.AsyncSuspendHybridClient:
        return _suspend_hybrid.AsyncSuspendHybridClient(
            self.client,
            self.path,
        )

    def suspend_ram(self) -> _suspend_ram.AsyncSuspendRamClient:
        return _suspend_ram.AsyncSuspendRamClient(
            self.client,
            self.path,
        )

    def set_user_password(self) -> _set_user_password.AsyncSetUserPasswordClient:
        return _set_user_password.AsyncSetUserPasswordClient(
            self.client,
            self.path,
        )

    def exec(self) -> _exec.AsyncExecClient:
        return _exec.AsyncExecClient(
            self.client,
            self.path,
        )

    def exec_status(self) -> _exec_status.AsyncExecStatusClient:
        return _exec_status.AsyncExecStatusClient(
            self.client,
            self.path,
        )

    def file_read(self) -> _file_read.AsyncFileReadClient:
        return _file_read.AsyncFileReadClient(
            self.client,
            self.path,
        )

    def file_write(self) -> _file_write.AsyncFileWriteClient:
        return _file_write.AsyncFileWriteClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        QEMU Guest Agent command index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Execute QEMU Guest Agent commands.
        """
        return await self.client.post(self.path, parameters, parse_as=PostResponseItem)
