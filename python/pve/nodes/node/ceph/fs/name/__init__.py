from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # Configure the created CephFS as storage for this cluster.
    add_storage: Optional[bool] = Field(alias="add-storage", default=None)
    # Number of placement groups for the backing data pool. The metadata pool will use a quarter of this.
    pg_num: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class NameClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    def post(self, parameters: PostParameters) -> str:
        """
        Create a Ceph filesystem
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncNameClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Create a Ceph filesystem
        """
        return await self.client.post(self.path, parameters, parse_as=str)
