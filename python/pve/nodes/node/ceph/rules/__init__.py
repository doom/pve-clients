from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    # Name of the CRUSH rule.
    name: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class RulesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'rules'}"

    def get(self) -> list[GetResponseItem]:
        """
        List ceph rules.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncRulesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'rules'}"

    async def get(self) -> list[GetResponseItem]:
        """
        List ceph rules.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
