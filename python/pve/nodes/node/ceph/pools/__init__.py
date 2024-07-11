from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import name as _name


class PostParameters(BaseModel):
    # Configure VM and CT storage using the new pool.
    add_storages: Optional[bool] = Field(default=None)
    # The application of the pool.
    application: Optional[str] = Field(default=None)
    # The rule to use for mapping object placement in the cluster.
    crush_rule: Optional[str] = Field(default=None)
    # Create an erasure coded pool for RBD with an accompaning replicated pool for metadata storage. With EC, the common ceph options 'size', 'min_size' and 'crush_rule' parameters will be applied to the metadata pool.
    erasure_coding: Optional[str] = Field(alias="erasure-coding", default=None)
    # Minimum number of replicas per object
    min_size: Optional[int] = Field(default=None)
    # The name of the pool. It must be unique.
    name: str
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


class AutoscaleStatus(BaseModel):
    pass


class ApplicationMetadata(BaseModel):
    pass


class GetResponseItem(BaseModel):
    application_metadata: Optional[ApplicationMetadata] = Field(default=None)
    autoscale_status: Optional[AutoscaleStatus] = Field(default=None)
    bytes_used: int
    crush_rule: int
    crush_rule_name: str
    min_size: int
    percent_used: float
    pg_autoscale_mode: Optional[str] = Field(default=None)
    pg_num: int
    pg_num_final: Optional[int] = Field(default=None)
    pg_num_min: Optional[int] = Field(default=None)
    pool: int
    pool_name: str
    size: int
    target_size: Optional[int] = Field(default=None)
    target_size_ratio: Optional[float] = Field(default=None)
    type: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class PoolsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'pools'}"

    def name(self, name: str) -> _name.NameClient:
        return _name.NameClient(
            self.client,
            self.path,
            name,
        )

    def get(self) -> list[GetResponseItem]:
        """
        List all pools. Deprecated, please use `/nodes/{node}/ceph/pool`.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters) -> str:
        """
        Create Ceph pool. Deprecated, please use `/nodes/{node}/ceph/pool`.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncPoolsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'pools'}"

    def name(self, name: str) -> _name.AsyncNameClient:
        return _name.AsyncNameClient(
            self.client,
            self.path,
            name,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        List all pools. Deprecated, please use `/nodes/{node}/ceph/pool`.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters) -> str:
        """
        Create Ceph pool. Deprecated, please use `/nodes/{node}/ceph/pool`.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
