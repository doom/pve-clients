from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


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


@dataclass
class StatusClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'status'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Show the current pool status.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncStatusClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'status'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Show the current pool status.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
