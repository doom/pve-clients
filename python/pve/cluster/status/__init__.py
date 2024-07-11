from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    id: str
    # [node] IP of the resolved nodename.
    ip: Optional[str] = Field(default=None)
    # [node] Proxmox VE Subscription level, indicates if eligible for enterprise support as well as access to the stable Proxmox VE Enterprise Repository.
    level: Optional[str] = Field(default=None)
    # [node] Indicates if this is the responding node.
    local: Optional[bool] = Field(default=None)
    name: str
    # [node] ID of the node from the corosync configuration.
    nodeid: Optional[int] = Field(default=None)
    # [cluster] Nodes count, including offline nodes.
    nodes: Optional[int] = Field(default=None)
    # [node] Indicates if the node is online or offline.
    online: Optional[bool] = Field(default=None)
    # [cluster] Indicates if there is a majority of nodes online to make decisions
    quorate: Optional[bool] = Field(default=None)
    # Indicates the type, either cluster or node. The type defines the object properties e.g. quorate available for type cluster.
    type: str
    # [cluster] Current version of the corosync configuration file.
    version: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class StatusClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'status'}"

    def get(self) -> list[GetResponseItem]:
        """
        Get cluster status information.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncStatusClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'status'}"

    async def get(self) -> list[GetResponseItem]:
        """
        Get cluster status information.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
