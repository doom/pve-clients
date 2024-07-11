from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    # Name of the guest
    name: Optional[str] = Field(default=None)
    # Type of the guest.
    type: str
    # VMID of the guest.
    vmid: int

    class Config(CommonPydanticConfig):
        pass


@dataclass
class NotBackedUpClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'not-backed-up'}"

    def get(self) -> list[GetResponseItem]:
        """
        Shows all guests which are not covered by any backup job.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncNotBackedUpClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'not-backed-up'}"

    async def get(self) -> list[GetResponseItem]:
        """
        Shows all guests which are not covered by any backup job.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
