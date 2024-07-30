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
    # Override QEMU's -cpu argument with the given string.
    force_cpu: Optional[str] = Field(alias="force-cpu", default=None)
    # Specify the QEMU machine.
    machine: Optional[str] = Field(default=None)
    # The cluster node name.
    migratedfrom: Optional[str] = Field(default=None)
    # CIDR of the (sub) network that is used for migration.
    migration_network: Optional[str] = Field(default=None)
    # Migration traffic is encrypted using an SSH tunnel by default. On secure, completely private networks this can be disabled to increase performance.
    migration_type: Optional[str] = Field(default=None)
    # Ignore locks - only root is allowed to use this option.
    skiplock: Optional[bool] = Field(default=None)
    # Some command save/restore state from this location.
    stateuri: Optional[str] = Field(default=None)
    # Mapping from source to target storages. Providing only a single storage ID maps all source storages to that storage. Providing the special value '1' will map each source storage to itself.
    targetstorage: Optional[str] = Field(default=None)
    # Wait maximal timeout seconds.
    timeout: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class StartClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'start'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Start virtual machine.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncStartClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'start'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Start virtual machine.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
