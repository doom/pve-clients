from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import volume as _volume


class PostParameters(BaseModel):
    # The name of the file to create.
    filename: str
    format: Optional[str] = Field(default=None)
    # Size in kilobyte (1024 bytes). Optional suffixes 'M' (megabyte, 1024K) and 'G' (gigabyte, 1024M)
    size: str
    # Specify owner VM
    vmid: int

    class Config(CommonPydanticConfig):
        pass


class Verification(BaseModel):
    """
    Last backup verification result, only useful for PBS storages.
    """

    # Last backup verification state.
    state: str
    # Last backup verification UPID.
    upid: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Creation time (seconds since the UNIX Epoch).
    ctime: Optional[int] = Field(default=None)
    # If whole backup is encrypted, value is the fingerprint or '1'  if encrypted. Only useful for the Proxmox Backup Server storage type.
    encrypted: Optional[str] = Field(default=None)
    # Format identifier ('raw', 'qcow2', 'subvol', 'iso', 'tgz' ...)
    format: str
    # Optional notes. If they contain multiple lines, only the first one is returned here.
    notes: Optional[str] = Field(default=None)
    # Volume identifier of parent (for linked cloned).
    parent: Optional[str] = Field(default=None)
    # Protection status. Currently only supported for backups.
    protected: Optional[bool] = Field(default=None)
    # Volume size in bytes.
    size: int
    # Used space. Please note that most storage plugins do not report anything useful here.
    used: Optional[int] = Field(default=None)
    # Last backup verification result, only useful for PBS storages.
    verification: Optional[Verification] = Field(default=None)
    # Associated Owner VMID.
    vmid: Optional[int] = Field(default=None)
    # Volume identifier.
    volid: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Only list content of this type.
    content: Optional[str] = Field(default=None)
    # Only list images for this VM
    vmid: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ContentClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'content'}"

    def volume(self, volume: str) -> _volume.VolumeClient:
        return _volume.VolumeClient(
            self.client,
            self.path,
            volume,
        )

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List storage content.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters) -> str:
        """
        Allocate disk images.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncContentClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'content'}"

    def volume(self, volume: str) -> _volume.AsyncVolumeClient:
        return _volume.AsyncVolumeClient(
            self.client,
            self.path,
            volume,
        )

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List storage content.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )

    async def post(self, parameters: PostParameters) -> str:
        """
        Allocate disk images.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
