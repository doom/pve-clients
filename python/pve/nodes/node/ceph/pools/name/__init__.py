from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


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


class Statistics(BaseModel):
    pass


class AutoscaleStatus(BaseModel):
    pass


class GetResponseItem(BaseModel):
    # The application of the pool.
    application: Optional[str] = Field(default=None)
    application_list: Optional[list[dict[str, Any]]] = Field(default=None)
    autoscale_status: Optional[AutoscaleStatus] = Field(default=None)
    # The rule to use for mapping object placement in the cluster.
    crush_rule: Optional[str] = Field(default=None)
    fast_read: bool
    hashpspool: bool
    id: int
    # Minimum number of replicas per object
    min_size: Optional[int] = Field(default=None)
    # The name of the pool. It must be unique.
    name: str
    nodeep_scrub: bool = Field(alias="nodeep-scrub")
    nodelete: bool
    nopgchange: bool
    noscrub: bool
    nosizechange: bool
    # The automatic PG scaling mode of the pool.
    pg_autoscale_mode: Optional[str] = Field(default=None)
    # Number of placement groups.
    pg_num: Optional[int] = Field(default=None)
    # Minimal number of placement groups.
    pg_num_min: Optional[int] = Field(default=None)
    pgp_num: int
    # Number of replicas per object
    size: Optional[int] = Field(default=None)
    statistics: Optional[Statistics] = Field(default=None)
    # The estimated target size of the pool for the PG autoscaler.
    target_size: Optional[str] = Field(default=None)
    # The estimated target ratio of the pool for the PG autoscaler.
    target_size_ratio: Optional[float] = Field(default=None)
    use_gmt_hitset: bool
    write_fadvise_dontneed: bool

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # If enabled, will display additional data(eg. statistics).
    verbose: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
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

    def delete(self, parameters: DeleteParameters) -> str:
        """
        Destroy pool. Deprecated, please use `/nodes/{node}/ceph/pool/{name}`.
        """
        return self.client.delete(self.path, parameters, parse_as=str)

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        List pool settings. Deprecated, please use `/nodes/{node}/ceph/pool/{pool}/status`.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters) -> str:
        """
        Change POOL settings. Deprecated, please use `/nodes/{node}/ceph/pool/{name}`.
        """
        return self.client.put(self.path, parameters, parse_as=str)


@dataclass
class AsyncNameClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    async def delete(self, parameters: DeleteParameters) -> str:
        """
        Destroy pool. Deprecated, please use `/nodes/{node}/ceph/pool/{name}`.
        """
        return await self.client.delete(self.path, parameters, parse_as=str)

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        List pool settings. Deprecated, please use `/nodes/{node}/ceph/pool/{pool}/status`.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters) -> str:
        """
        Change POOL settings. Deprecated, please use `/nodes/{node}/ceph/pool/{name}`.
        """
        return await self.client.put(self.path, parameters, parse_as=str)
