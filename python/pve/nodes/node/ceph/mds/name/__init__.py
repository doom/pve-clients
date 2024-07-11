from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # Determines whether a ceph-mds daemon should poll and replay the log of an active MDS. Faster switch on MDS failure, but needs more idle resources.
    hotstandby: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class NameClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    def delete(self) -> str:
        """
        Destroy Ceph Metadata Server
        """
        return self.client.delete(self.path, parse_as=str)

    def post(self, parameters: PostParameters) -> str:
        """
        Create Ceph Metadata Server (MDS)
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncNameClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    async def delete(self) -> str:
        """
        Destroy Ceph Metadata Server
        """
        return await self.client.delete(self.path, parse_as=str)

    async def post(self, parameters: PostParameters) -> str:
        """
        Create Ceph Metadata Server (MDS)
        """
        return await self.client.post(self.path, parameters, parse_as=str)
