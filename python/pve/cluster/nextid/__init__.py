from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetParameters(BaseModel):
    # The (unique) ID of the VM.
    vmid: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class NextidClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'nextid'}"

    def get(self, parameters: GetParameters) -> int:
        """
        Get next free VMID. Pass a VMID to assert that its free (at time of check).
        """
        return self.client.get(self.path, parameters, parse_as=int)


@dataclass
class AsyncNextidClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'nextid'}"

    async def get(self, parameters: GetParameters) -> int:
        """
        Get next free VMID. Pass a VMID to assert that its free (at time of check).
        """
        return await self.client.get(self.path, parameters, parse_as=int)
