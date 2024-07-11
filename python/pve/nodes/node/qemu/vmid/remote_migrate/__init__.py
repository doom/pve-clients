from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # Override I/O bandwidth limit (in KiB/s).
    bwlimit: Optional[int] = Field(default=None)
    # Delete the original VM and related data after successful migration. By default the original VM is kept on the source cluster in a stopped state.
    delete: Optional[bool] = Field(default=None)
    # Use online/live migration if VM is running. Ignored if VM is stopped.
    online: Optional[bool] = Field(default=None)
    # Mapping from source to target bridges. Providing only a single bridge ID maps all source bridges to that bridge. Providing the special value '1' will map each source bridge to itself.
    target_bridge: str = Field(alias="target-bridge")
    # Remote target endpoint
    target_endpoint: str = Field(alias="target-endpoint")
    # Mapping from source to target storages. Providing only a single storage ID maps all source storages to that storage. Providing the special value '1' will map each source storage to itself.
    target_storage: str = Field(alias="target-storage")
    # The (unique) ID of the VM.
    target_vmid: Optional[int] = Field(alias="target-vmid", default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class RemoteMigrateClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'remote_migrate'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Migrate virtual machine to a remote cluster. Creates a new migration task. EXPERIMENTAL feature!
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncRemoteMigrateClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'remote_migrate'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Migrate virtual machine to a remote cluster. Creates a new migration task. EXPERIMENTAL feature!
        """
        return await self.client.post(self.path, parameters, parse_as=str)
