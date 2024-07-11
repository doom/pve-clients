from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    can_update_at_runtime: bool
    level: str
    mask: str
    name: str
    section: str
    value: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ConfigdbClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'configdb'}"

    def get(self) -> list[GetResponseItem]:
        """
        Get the Ceph configuration database. Deprecated, please use `/nodes/{node}/ceph/cfg/db.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncConfigdbClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'configdb'}"

    async def get(self) -> list[GetResponseItem]:
        """
        Get the Ceph configuration database. Deprecated, please use `/nodes/{node}/ceph/cfg/db.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
