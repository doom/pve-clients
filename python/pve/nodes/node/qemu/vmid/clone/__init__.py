from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # Override I/O bandwidth limit (in KiB/s).
    bwlimit: Optional[int] = Field(default=None)
    # Description for the new VM.
    description: Optional[str] = Field(default=None)
    # Target format for file storage. Only valid for full clone.
    format: Optional[str] = Field(default=None)
    # Create a full copy of all disks. This is always done when you clone a normal VM. For VM templates, we try to create a linked clone by default.
    full: Optional[bool] = Field(default=None)
    # Set a name for the new VM.
    name: Optional[str] = Field(default=None)
    # VMID for the clone.
    newid: int
    # Add the new VM to the specified pool.
    pool: Optional[str] = Field(default=None)
    # The name of the snapshot.
    snapname: Optional[str] = Field(default=None)
    # Target storage for full clone.
    storage: Optional[str] = Field(default=None)
    # Target node. Only allowed if the original VM is on shared storage.
    target: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class CloneClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'clone'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Create a copy of virtual machine/template.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncCloneClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'clone'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Create a copy of virtual machine/template.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
