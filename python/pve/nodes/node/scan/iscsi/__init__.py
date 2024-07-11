from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    # The iSCSI portal name.
    portal: str
    # The iSCSI target name.
    target: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # The iSCSI portal (IP or DNS name with optional port).
    portal: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class IscsiClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'iscsi'}"

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Scan remote iSCSI server.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncIscsiClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'iscsi'}"

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Scan remote iSCSI server.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
