from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import config as _config
from . import rollback as _rollback


class GetResponseItem(BaseModel):
    pass


class DeleteParameters(BaseModel):
    # For removal from config file, even if removing disk snapshots fails.
    force: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class SnapnameClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, snapname: str):
        self.client = client
        self.path = f"{parent_path}/{snapname}"

    def rollback(self) -> _rollback.RollbackClient:
        return _rollback.RollbackClient(
            self.client,
            self.path,
        )

    def config(self) -> _config.ConfigClient:
        return _config.ConfigClient(
            self.client,
            self.path,
        )

    def delete(self, parameters: DeleteParameters) -> str:
        """
        Delete a LXC snapshot.
        """
        return self.client.delete(self.path, parameters, parse_as=str)

    def get(self) -> list[GetResponseItem]:
        """ """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncSnapnameClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, snapname: str):
        self.client = client
        self.path = f"{parent_path}/{snapname}"

    def rollback(self) -> _rollback.AsyncRollbackClient:
        return _rollback.AsyncRollbackClient(
            self.client,
            self.path,
        )

    def config(self) -> _config.AsyncConfigClient:
        return _config.AsyncConfigClient(
            self.client,
            self.path,
        )

    async def delete(self, parameters: DeleteParameters) -> str:
        """
        Delete a LXC snapshot.
        """
        return await self.client.delete(self.path, parameters, parse_as=str)

    async def get(self) -> list[GetResponseItem]:
        """ """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
