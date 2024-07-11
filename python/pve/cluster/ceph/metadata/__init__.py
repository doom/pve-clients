from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class Node(BaseModel):
    """
    Ceph version installed on the nodes.
    """

    pass


class Mon(BaseModel):
    """
    Monitors configured in the cluster and their properties.
    """

    pass


class Mgr(BaseModel):
    """
    Managers configured in the cluster and their properties.
    """

    pass


class Mds(BaseModel):
    """
    Metadata servers configured in the cluster and their properties.
    """

    pass


class GetResponseItem(BaseModel):
    """
    Items for each type of service containing objects for each instance.
    """

    # Metadata servers configured in the cluster and their properties.
    mds: Mds
    # Managers configured in the cluster and their properties.
    mgr: Mgr
    # Monitors configured in the cluster and their properties.
    mon: Mon
    # Ceph version installed on the nodes.
    node: Node
    # OSDs configured in the cluster and their properties.
    osd: list[dict[str, Any]]

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    scope: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MetadataClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'metadata'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get ceph metadata.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncMetadataClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'metadata'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get ceph metadata.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
