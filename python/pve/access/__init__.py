from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import acl as _acl
from . import domains as _domains
from . import groups as _groups
from . import openid as _openid
from . import password as _password
from . import permissions as _permissions
from . import roles as _roles
from . import tfa as _tfa
from . import ticket as _ticket
from . import users as _users


class GetResponseItem(BaseModel):
    subdir: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class AccessClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient):
        self.client = client
        self.path = "access"

    def users(self) -> _users.UsersClient:
        return _users.UsersClient(
            self.client,
            self.path,
        )

    def groups(self) -> _groups.GroupsClient:
        return _groups.GroupsClient(
            self.client,
            self.path,
        )

    def roles(self) -> _roles.RolesClient:
        return _roles.RolesClient(
            self.client,
            self.path,
        )

    def acl(self) -> _acl.AclClient:
        return _acl.AclClient(
            self.client,
            self.path,
        )

    def domains(self) -> _domains.DomainsClient:
        return _domains.DomainsClient(
            self.client,
            self.path,
        )

    def openid(self) -> _openid.OpenidClient:
        return _openid.OpenidClient(
            self.client,
            self.path,
        )

    def tfa(self) -> _tfa.TfaClient:
        return _tfa.TfaClient(
            self.client,
            self.path,
        )

    def ticket(self) -> _ticket.TicketClient:
        return _ticket.TicketClient(
            self.client,
            self.path,
        )

    def password(self) -> _password.PasswordClient:
        return _password.PasswordClient(
            self.client,
            self.path,
        )

    def permissions(self) -> _permissions.PermissionsClient:
        return _permissions.PermissionsClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncAccessClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient):
        self.client = client
        self.path = "access"

    def users(self) -> _users.AsyncUsersClient:
        return _users.AsyncUsersClient(
            self.client,
            self.path,
        )

    def groups(self) -> _groups.AsyncGroupsClient:
        return _groups.AsyncGroupsClient(
            self.client,
            self.path,
        )

    def roles(self) -> _roles.AsyncRolesClient:
        return _roles.AsyncRolesClient(
            self.client,
            self.path,
        )

    def acl(self) -> _acl.AsyncAclClient:
        return _acl.AsyncAclClient(
            self.client,
            self.path,
        )

    def domains(self) -> _domains.AsyncDomainsClient:
        return _domains.AsyncDomainsClient(
            self.client,
            self.path,
        )

    def openid(self) -> _openid.AsyncOpenidClient:
        return _openid.AsyncOpenidClient(
            self.client,
            self.path,
        )

    def tfa(self) -> _tfa.AsyncTfaClient:
        return _tfa.AsyncTfaClient(
            self.client,
            self.path,
        )

    def ticket(self) -> _ticket.AsyncTicketClient:
        return _ticket.AsyncTicketClient(
            self.client,
            self.path,
        )

    def password(self) -> _password.AsyncPasswordClient:
        return _password.AsyncPasswordClient(
            self.client,
            self.path,
        )

    def permissions(self) -> _permissions.AsyncPermissionsClient:
        return _permissions.AsyncPermissionsClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
