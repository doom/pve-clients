from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import migrate as _migrate
from . import relocate as _relocate


class PutParameters(BaseModel):
    # Description.
    comment: Optional[str] = Field(default=None)
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # The HA group identifier.
    group: Optional[str] = Field(default=None)
    # Maximal number of service relocate tries when a service failes to start.
    max_relocate: Optional[int] = Field(default=None)
    # Maximal number of tries to restart the service on a node after its start failed.
    max_restart: Optional[int] = Field(default=None)
    # Requested resource state.
    state: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Description.
    comment: Optional[str] = Field(default=None)
    # Can be used to prevent concurrent modifications.
    digest: str
    # The HA group identifier.
    group: Optional[str] = Field(default=None)
    # Maximal number of service relocate tries when a service failes to start.
    max_relocate: Optional[int] = Field(default=None)
    # Maximal number of tries to restart the service on a node after its start failed.
    max_restart: Optional[int] = Field(default=None)
    # HA resource ID. This consists of a resource type followed by a resource specific name, separated with colon (example: vm:100 / ct:100). For virtual machines and containers, you can simply use the VM or CT id as a shortcut (example: 100).
    sid: str
    # Requested resource state.
    state: Optional[str] = Field(default=None)
    # The type of the resources.
    type: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class SidClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, sid: str):
        self.client = client
        self.path = f"{parent_path}/{sid}"

    def migrate(self) -> _migrate.MigrateClient:
        return _migrate.MigrateClient(
            self.client,
            self.path,
        )

    def relocate(self) -> _relocate.RelocateClient:
        return _relocate.RelocateClient(
            self.client,
            self.path,
        )

    def delete(self):
        """
        Delete resource configuration.
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> GetResponseItem:
        """
        Read resource configuration.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update resource configuration.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncSidClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, sid: str):
        self.client = client
        self.path = f"{parent_path}/{sid}"

    def migrate(self) -> _migrate.AsyncMigrateClient:
        return _migrate.AsyncMigrateClient(
            self.client,
            self.path,
        )

    def relocate(self) -> _relocate.AsyncRelocateClient:
        return _relocate.AsyncRelocateClient(
            self.client,
            self.path,
        )

    async def delete(self):
        """
        Delete resource configuration.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> GetResponseItem:
        """
        Read resource configuration.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update resource configuration.
        """
        return await self.client.put(self.path, parameters)
