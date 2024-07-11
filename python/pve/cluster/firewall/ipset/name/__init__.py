from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import cidr as _cidr


class PostParameters(BaseModel):
    # Network/IP specification in CIDR format.
    cidr: str
    comment: Optional[str] = Field(default=None)
    nomatch: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    cidr: str
    comment: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: str
    nomatch: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class DeleteParameters(BaseModel):
    # Delete all members of the IPSet, if there are any.
    force: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class NameClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    def cidr(self, cidr: str) -> _cidr.CidrClient:
        return _cidr.CidrClient(
            self.client,
            self.path,
            cidr,
        )

    def delete(self, parameters: DeleteParameters):
        """
        Delete IPSet
        """
        return self.client.delete(self.path, parameters)

    def get(self) -> list[GetResponseItem]:
        """
        List IPSet content
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Add IP or Network to IPSet.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncNameClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    def cidr(self, cidr: str) -> _cidr.AsyncCidrClient:
        return _cidr.AsyncCidrClient(
            self.client,
            self.path,
            cidr,
        )

    async def delete(self, parameters: DeleteParameters):
        """
        Delete IPSet
        """
        return await self.client.delete(self.path, parameters)

    async def get(self) -> list[GetResponseItem]:
        """
        List IPSet content
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters):
        """
        Add IP or Network to IPSet.
        """
        return await self.client.post(self.path, parameters)
