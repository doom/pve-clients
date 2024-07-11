from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import roleid as _roleid


class PostParameters(BaseModel):
    privs: Optional[str] = Field(default=None)
    roleid: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    privs: Optional[str] = Field(default=None)
    roleid: str
    special: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class RolesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'roles'}"

    def roleid(self, roleid: str) -> _roleid.RoleidClient:
        return _roleid.RoleidClient(
            self.client,
            self.path,
            roleid,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Role index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create new role.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncRolesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'roles'}"

    def roleid(self, roleid: str) -> _roleid.AsyncRoleidClient:
        return _roleid.AsyncRoleidClient(
            self.client,
            self.path,
            roleid,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Role index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters):
        """
        Create new role.
        """
        return await self.client.post(self.path, parameters)
