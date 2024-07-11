from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import aliases as _aliases
from . import ipset as _ipset
from . import log as _log
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

    def rules(self) -> _rules.RulesClient:
        return _rules.RulesClient(
            self.client,
            self.path,
        )

    def aliases(self) -> _aliases.AliasesClient:
        return _aliases.AliasesClient(
            self.client,
            self.path,
        )

    def ipset(self) -> _ipset.IpsetClient:
        return _ipset.IpsetClient(
            self.client,
            self.path,
        )

    def options(self) -> _options.OptionsClient:
        return _options.OptionsClient(
            self.client,
            self.path,
        )

    def log(self) -> _log.LogClient:
        return _log.LogClient(
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

    def rules(self) -> _rules.AsyncRulesClient:
        return _rules.AsyncRulesClient(
            self.client,
            self.path,
        )

    def aliases(self) -> _aliases.AsyncAliasesClient:
        return _aliases.AsyncAliasesClient(
            self.client,
            self.path,
        )

    def ipset(self) -> _ipset.AsyncIpsetClient:
        return _ipset.AsyncIpsetClient(
            self.client,
            self.path,
        )

    def options(self) -> _options.AsyncOptionsClient:
        return _options.AsyncOptionsClient(
            self.client,
            self.path,
        )

    def log(self) -> _log.AsyncLogClient:
        return _log.AsyncLogClient(
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
