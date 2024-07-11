from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    # Comment from server.
    comment: Optional[str] = Field(default=None)
    # The datastore name.
    store: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Certificate SHA 256 fingerprint.
    fingerprint: Optional[str] = Field(default=None)
    # User password or API token secret.
    password: str
    # Optional port.
    port: Optional[int] = Field(default=None)
    # The server address (name or IP).
    server: str
    # User-name or API token-ID.
    username: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class PbsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'pbs'}"

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Scan remote Proxmox Backup Server.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncPbsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'pbs'}"

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Scan remote Proxmox Backup Server.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
