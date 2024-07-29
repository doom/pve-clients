from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PostParameters(BaseModel):
    # Override I/O bandwidth limit (in KiB/s).
    bwlimit: Optional[int] = Field(default=None)
    # Delete the original disk after successful copy. By default the original disk is kept as unused disk.
    delete: Optional[bool] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1\" 		    .\" digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # The disk you want to move.
    disk: str
    # Target Format.
    format: Optional[str] = Field(default=None)
    # Target storage.
    storage: Optional[str] = Field(default=None)
    # Prevent changes if the current config file of the target VM has a\" 		    .\" different SHA1 digest. This can be used to detect concurrent modifications.
    target_digest: Optional[str] = Field(alias="target-digest", default=None)
    # The config key the disk will be moved to on the target VM (for example, ide0 or scsi1). Default is the source disk key.
    target_disk: Optional[str] = Field(alias="target-disk", default=None)
    # The (unique) ID of the VM.
    target_vmid: Optional[int] = Field(alias="target-vmid", default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MoveDiskClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'move_disk'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Move volume to different storage or to a different VM.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncMoveDiskClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'move_disk'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Move volume to different storage or to a different VM.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
