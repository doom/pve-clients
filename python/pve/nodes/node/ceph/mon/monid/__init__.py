from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # Overwrites autodetected monitor IP address(es). Must be in the public network(s) of Ceph.
    mon_address: Optional[str] = Field(alias="mon-address", default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MonidClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, monid: str):
        self.client = client
        self.path = f"{parent_path}/{monid}"

    def delete(self) -> str:
        """
        Destroy Ceph Monitor and Manager.
        """
        return self.client.delete(self.path, parse_as=str)

    def post(self, parameters: PostParameters) -> str:
        """
        Create Ceph Monitor and Manager
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncMonidClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, monid: str):
        self.client = client
        self.path = f"{parent_path}/{monid}"

    async def delete(self) -> str:
        """
        Destroy Ceph Monitor and Manager.
        """
        return await self.client.delete(self.path, parse_as=str)

    async def post(self, parameters: PostParameters) -> str:
        """
        Create Ceph Monitor and Manager
        """
        return await self.client.post(self.path, parameters, parse_as=str)
