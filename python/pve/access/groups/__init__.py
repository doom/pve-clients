from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import groupid as _groupid


class PostParameters(BaseModel):
    comment: Optional[str] = Field(default=None)
    groupid: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    comment: Optional[str] = Field(default=None)
    groupid: str
    # list of users which form this group
    users: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class GroupsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'groups'}"

    def groupid(self, groupid: str) -> _groupid.GroupidClient:
        return _groupid.GroupidClient(
            self.client,
            self.path,
            groupid,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Group index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create new group.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncGroupsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'groups'}"

    def groupid(self, groupid: str) -> _groupid.AsyncGroupidClient:
        return _groupid.AsyncGroupidClient(
            self.client,
            self.path,
            groupid,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Group index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters):
        """
        Create new group.
        """
        return await self.client.post(self.path, parameters)
