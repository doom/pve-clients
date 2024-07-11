from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    # NFS export options.
    options: str
    # The exported path.
    path: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # The server address (name or IP).
    server: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class NfsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'nfs'}"

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Scan remote NFS server.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncNfsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'nfs'}"

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Scan remote NFS server.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
