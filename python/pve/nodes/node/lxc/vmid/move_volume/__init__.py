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
    bwlimit: Optional[float] = Field(default=None)
    # Delete the original volume after successful copy. By default the original is kept as an unused volume entry.
    delete: Optional[bool] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 \" . 		    \"digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Target Storage.
    storage: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file of the target \" . 		    \"container has a different SHA1 digest. This can be used to prevent \" . 		    \"concurrent modifications.
    target_digest: Optional[str] = Field(alias="target-digest", default=None)
    # The (unique) ID of the VM.
    target_vmid: Optional[int] = Field(alias="target-vmid", default=None)
    # The config key the volume will be moved to. Default is the source volume key.
    target_volume: Optional[str] = Field(alias="target-volume", default=None)
    # Volume which will be moved.
    volume: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MoveVolumeClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'move_volume'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Move a rootfs-/mp-volume to a different storage or to a different container.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncMoveVolumeClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'move_volume'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Move a rootfs-/mp-volume to a different storage or to a different container.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
