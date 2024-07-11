from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    # The LVM Thin Pool name (LVM logical volume).
    lv: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    vg: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class LvmthinClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'lvmthin'}"

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List local LVM Thin Pools.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncLvmthinClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'lvmthin'}"

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List local LVM Thin Pools.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
