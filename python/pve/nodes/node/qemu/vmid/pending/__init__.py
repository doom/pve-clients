from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetResponseItem(BaseModel):
    # Indicates a pending delete request if present and not 0. The value 2 indicates a force-delete request.
    delete: Optional[int] = Field(default=None)
    # Configuration option name.
    key: str
    # Pending value.
    pending: Optional[str] = Field(default=None)
    # Current value.
    value: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class PendingClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'pending'}"

    def get(self) -> list[GetResponseItem]:
        """
        Get the virtual machine configuration with both current and pending values.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncPendingClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'pending'}"

    async def get(self) -> list[GetResponseItem]:
        """
        Get the virtual machine configuration with both current and pending values.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
