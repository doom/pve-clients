from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # Declare a separate cluster network, OSDs will routeheartbeat, object replication and recovery traffic over it
    cluster_network: Optional[str] = Field(alias="cluster-network", default=None)
    # Disable cephx authentication.  WARNING: cephx is a security feature protecting against man-in-the-middle attacks. Only consider disabling cephx if your network is private!
    disable_cephx: Optional[bool] = Field(default=None)
    # Minimum number of available replicas per object to allow I/O
    min_size: Optional[int] = Field(default=None)
    # Use specific network for all ceph related traffic
    network: Optional[str] = Field(default=None)
    # Placement group bits, used to specify the default number of placement groups.  NOTE: 'osd pool default pg num' does not work for default pools.
    pg_bits: Optional[int] = Field(default=None)
    # Targeted number of replicas per object
    size: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class InitClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'init'}"

    def post(self, parameters: PostParameters):
        """
        Create initial ceph default configuration and setup symlinks.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncInitClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'init'}"

    async def post(self, parameters: PostParameters):
        """
        Create initial ceph default configuration and setup symlinks.
        """
        return await self.client.post(self.path, parameters)
