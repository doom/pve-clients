from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import group as _group


class PostParameters(BaseModel):
    comment: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Security Group name.
    group: str
    # Rename/update an existing security group. You can set 'rename' to the same value as 'name' to update the 'comment' of an existing group.
    rename: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    comment: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: str
    # Security Group name.
    group: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class GroupsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'groups'}"

    def group(self, group: str) -> _group.GroupClient:
        return _group.GroupClient(
            self.client,
            self.path,
            group,
        )

    def get(self) -> list[GetResponseItem]:
        """
        List security groups.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create new security group.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncGroupsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'groups'}"

    def group(self, group: str) -> _group.AsyncGroupClient:
        return _group.AsyncGroupClient(
            self.client,
            self.path,
            group,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        List security groups.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters):
        """
        Create new security group.
        """
        return await self.client.post(self.path, parameters)
