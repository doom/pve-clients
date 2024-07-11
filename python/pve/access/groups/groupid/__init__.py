from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PutParameters(BaseModel):
    comment: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    comment: Optional[str] = Field(default=None)
    members: list[str]

    class Config(CommonPydanticConfig):
        pass


@dataclass
class GroupidClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, groupid: str):
        self.client = client
        self.path = f"{parent_path}/{groupid}"

    def delete(self):
        """
        Delete group.
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> GetResponseItem:
        """
        Get group configuration.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update group data.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncGroupidClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, groupid: str):
        self.client = client
        self.path = f"{parent_path}/{groupid}"

    async def delete(self):
        """
        Delete group.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> GetResponseItem:
        """
        Get group configuration.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update group data.
        """
        return await self.client.put(self.path, parameters)
