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
    # Whether the VM should get started after rolling back successfully. (Note: VMs will be automatically started if the snapshot includes RAM.)
    start: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class RollbackClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'rollback'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Rollback VM state to specified snapshot.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncRollbackClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'rollback'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Rollback VM state to specified snapshot.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
