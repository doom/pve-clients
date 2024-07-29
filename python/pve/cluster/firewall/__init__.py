from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import aliases as _aliases
from . import groups as _groups
from . import ipset as _ipset
from . import macros as _macros
from . import options as _options
from . import refs as _refs
from . import rules as _rules


class GetResponseItem(BaseModel):
    pass


@dataclass
class FirewallClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'firewall'}"

    def groups(self) -> _groups.GroupsClient:
        return _groups.GroupsClient(
            self.client,
            self.path,
        )

    def rules(self) -> _rules.RulesClient:
        return _rules.RulesClient(
            self.client,
            self.path,
        )

    def ipset(self) -> _ipset.IpsetClient:
        return _ipset.IpsetClient(
            self.client,
            self.path,
        )

    def aliases(self) -> _aliases.AliasesClient:
        return _aliases.AliasesClient(
            self.client,
            self.path,
        )

    def options(self) -> _options.OptionsClient:
        return _options.OptionsClient(
            self.client,
            self.path,
        )

    def macros(self) -> _macros.MacrosClient:
        return _macros.MacrosClient(
            self.client,
            self.path,
        )

    def refs(self) -> _refs.RefsClient:
        return _refs.RefsClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncFirewallClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'firewall'}"

    def groups(self) -> _groups.AsyncGroupsClient:
        return _groups.AsyncGroupsClient(
            self.client,
            self.path,
        )

    def rules(self) -> _rules.AsyncRulesClient:
        return _rules.AsyncRulesClient(
            self.client,
            self.path,
        )

    def ipset(self) -> _ipset.AsyncIpsetClient:
        return _ipset.AsyncIpsetClient(
            self.client,
            self.path,
        )

    def aliases(self) -> _aliases.AsyncAliasesClient:
        return _aliases.AsyncAliasesClient(
            self.client,
            self.path,
        )

    def options(self) -> _options.AsyncOptionsClient:
        return _options.AsyncOptionsClient(
            self.client,
            self.path,
        )

    def macros(self) -> _macros.AsyncMacrosClient:
        return _macros.AsyncMacrosClient(
            self.client,
            self.path,
        )

    def refs(self) -> _refs.AsyncRefsClient:
        return _refs.AsyncRefsClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
