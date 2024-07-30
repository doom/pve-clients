from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import name as _name


class PostParameters(BaseModel):
    comment: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # IP set name.
    name: str
    # Rename an existing IPSet. You can set 'rename' to the same value as 'name' to update the 'comment' of an existing IPSet.
    rename: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    comment: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
    digest: str
    # IP set name.
    name: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class IpsetClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'ipset'}"

    def name(self, name: str) -> _name.NameClient:
        return _name.NameClient(
            self.client,
            self.path,
            name,
        )

    def get(self) -> list[GetResponseItem]:
        """
        List IPSets
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create new IPSet
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncIpsetClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'ipset'}"

    def name(self, name: str) -> _name.AsyncNameClient:
        return _name.AsyncNameClient(
            self.client,
            self.path,
            name,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        List IPSets
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters):
        """
        Create new IPSet
        """
        return await self.client.post(self.path, parameters)
