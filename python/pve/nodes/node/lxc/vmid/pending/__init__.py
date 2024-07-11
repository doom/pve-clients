from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    # Indicates a pending delete request if present and not 0.
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
        Get container configuration, including pending changes.
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
        Get container configuration, including pending changes.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
