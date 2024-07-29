from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import sid as _sid


class PostParameters(BaseModel):
    # Description.
    comment: Optional[str] = Field(default=None)
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
    # Resource type.
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    sid: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Only list resources of specific type
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ResourcesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'resources'}"

    def sid(self, sid: str) -> _sid.SidClient:
        return _sid.SidClient(
            self.client,
            self.path,
            sid,
        )

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List HA resources.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create a new HA resource.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncResourcesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'resources'}"

    def sid(self, sid: str) -> _sid.AsyncSidClient:
        return _sid.AsyncSidClient(
            self.client,
            self.path,
            sid,
        )

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List HA resources.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )

    async def post(self, parameters: PostParameters):
        """
        Create a new HA resource.
        """
        return await self.client.post(self.path, parameters)
