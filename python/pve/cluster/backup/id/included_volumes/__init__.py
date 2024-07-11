from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseChildrenItemChildrenItem(BaseModel):
    # Configuration key of the volume.
    id: str
    # Whether the volume is included in the backup or not.
    included: bool
    # Name of the volume.
    name: str
    # The reason why the volume is included (or excluded).
    reason: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseChildrenItem(BaseModel):
    # The volumes of the guest with the information if they will be included in backups.
    children: Optional[list[GetResponseChildrenItemChildrenItem]] = Field(default=None)
    # VMID of the guest.
    id: int
    # Name of the guest
    name: Optional[str] = Field(default=None)
    # Type of the guest, VM, CT or unknown for removed but not purged guests.
    type: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    """
    Root node of the tree object. Children represent guests, grandchildren represent volumes of that guest.
    """

    children: list[GetResponseChildrenItem]

    class Config(CommonPydanticConfig):
        pass


@dataclass
class IncludedVolumesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'included_volumes'}"

    def get(self) -> GetResponseItem:
        """
        Returns included guests and the backup status of their disks. Optimized to be used in ExtJS tree views.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)


@dataclass
class AsyncIncludedVolumesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'included_volumes'}"

    async def get(self) -> GetResponseItem:
        """
        Returns included guests and the backup status of their disks. Optimized to be used in ExtJS tree views.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)
