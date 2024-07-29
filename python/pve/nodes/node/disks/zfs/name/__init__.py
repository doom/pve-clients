from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetResponseChildrenItem(BaseModel):
    cksum: Optional[float] = Field(default=None)
    # An optional message about the vdev.
    msg: str
    # The name of the vdev or section.
    name: str
    read: Optional[float] = Field(default=None)
    # The state of the vdev.
    state: Optional[str] = Field(default=None)
    write: Optional[float] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Information about the recommended action to fix the state.
    action: Optional[str] = Field(default=None)
    # The pool configuration information, including the vdevs for each section (e.g. spares, cache), may be nested.
    children: list[GetResponseChildrenItem]
    # Information about the errors on the zpool.
    errors: str
    # The name of the zpool.
    name: str
    # Information about the last/current scrub.
    scan: Optional[str] = Field(default=None)
    # The state of the zpool.
    state: str
    # Information about the state of the zpool.
    status: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class DeleteParameters(BaseModel):
    # Marks associated storage(s) as not available on this node anymore or removes them from the configuration (if configured for this node only).
    cleanup_config: Optional[bool] = Field(alias="cleanup-config", default=None)
    # Also wipe disks so they can be repurposed afterwards.
    cleanup_disks: Optional[bool] = Field(alias="cleanup-disks", default=None)

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
        Destroy a ZFS pool.
        """
        return self.client.delete(self.path, parameters, parse_as=str)

    def get(self) -> GetResponseItem:
        """
        Get details about a zpool.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)


@dataclass
class AsyncNameClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    async def delete(self, parameters: DeleteParameters) -> str:
        """
        Destroy a ZFS pool.
        """
        return await self.client.delete(self.path, parameters, parse_as=str)

    async def get(self) -> GetResponseItem:
        """
        Get details about a zpool.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)
