from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    # The LVM logical volume group name.
    vg: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class LvmClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'lvm'}"

    def get(self) -> list[GetResponseItem]:
        """
        List local LVM volume groups.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncLvmClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'lvm'}"

    async def get(self) -> list[GetResponseItem]:
        """
        List local LVM volume groups.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
