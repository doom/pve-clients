from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import status as _status


class PutParameters(BaseModel):
    # The application of the pool.
    application: Optional[str] = Field(default=None)
    # The rule to use for mapping object placement in the cluster.
    crush_rule: Optional[str] = Field(default=None)
    # Minimum number of replicas per object
    min_size: Optional[int] = Field(default=None)
    # The automatic PG scaling mode of the pool.
    pg_autoscale_mode: Optional[str] = Field(default=None)
    # Number of placement groups.
    pg_num: Optional[int] = Field(default=None)
    # Minimal number of placement groups.
    pg_num_min: Optional[int] = Field(default=None)
    # Number of replicas per object
    size: Optional[int] = Field(default=None)
    # The estimated target size of the pool for the PG autoscaler.
    target_size: Optional[str] = Field(default=None)
    # The estimated target ratio of the pool for the PG autoscaler.
    target_size_ratio: Optional[float] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


class DeleteParameters(BaseModel):
    # If true, destroys pool even if in use
    force: Optional[bool] = Field(default=None)
    # Remove the erasure code profile. Defaults to true, if applicable.
    remove_ecprofile: Optional[bool] = Field(default=None)
    # Remove all pveceph-managed storages configured for this pool
    remove_storages: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class NameClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    def status(self) -> _status.StatusClient:
        return _status.StatusClient(
            self.client,
            self.path,
        )

    def delete(self, parameters: DeleteParameters) -> str:
        """
        Destroy pool
        """
        return self.client.delete(self.path, parameters, parse_as=str)

    def get(self) -> list[GetResponseItem]:
        """
        Pool index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def put(self, parameters: PutParameters) -> str:
        """
        Change POOL settings
        """
        return self.client.put(self.path, parameters, parse_as=str)


@dataclass
class AsyncNameClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    def status(self) -> _status.AsyncStatusClient:
        return _status.AsyncStatusClient(
            self.client,
            self.path,
        )

    async def delete(self, parameters: DeleteParameters) -> str:
        """
        Destroy pool
        """
        return await self.client.delete(self.path, parameters, parse_as=str)

    async def get(self) -> list[GetResponseItem]:
        """
        Pool index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def put(self, parameters: PutParameters) -> str:
        """
        Change POOL settings
        """
        return await self.client.put(self.path, parameters, parse_as=str)
