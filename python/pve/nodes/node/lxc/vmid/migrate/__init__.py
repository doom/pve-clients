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
    # Use online/live migration.
    online: Optional[bool] = Field(default=None)
    # Use restart migration
    restart: Optional[bool] = Field(default=None)
    # Target node.
    target: str
    # Mapping from source to target storages. Providing only a single storage ID maps all source storages to that storage. Providing the special value '1' will map each source storage to itself.
    target_storage: Optional[str] = Field(alias="target-storage", default=None)
    # Timeout in seconds for shutdown for restart migration
    timeout: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MigrateClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'migrate'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Migrate the container to another node. Creates a new migration task.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncMigrateClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'migrate'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Migrate the container to another node. Creates a new migration task.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
