from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import groups as _groups
from . import resources as _resources
from . import status as _status


class GetResponseItem(BaseModel):
    id: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class HaClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'ha'}"

    def resources(self) -> _resources.ResourcesClient:
        return _resources.ResourcesClient(
            self.client,
            self.path,
        )

    def groups(self) -> _groups.GroupsClient:
        return _groups.GroupsClient(
            self.client,
            self.path,
        )

    def status(self) -> _status.StatusClient:
        return _status.StatusClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncHaClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'ha'}"

    def resources(self) -> _resources.AsyncResourcesClient:
        return _resources.AsyncResourcesClient(
            self.client,
            self.path,
        )

    def groups(self) -> _groups.AsyncGroupsClient:
        return _groups.AsyncGroupsClient(
            self.client,
            self.path,
        )

    def status(self) -> _status.AsyncStatusClient:
        return _status.AsyncStatusClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
