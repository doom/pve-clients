from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import controllers as _controllers
from . import dns as _dns
from . import ipams as _ipams
from . import vnets as _vnets
from . import zones as _zones


class GetResponseItem(BaseModel):
    id: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class SdnClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'sdn'}"

    def vnets(self) -> _vnets.VnetsClient:
        return _vnets.VnetsClient(
            self.client,
            self.path,
        )

    def zones(self) -> _zones.ZonesClient:
        return _zones.ZonesClient(
            self.client,
            self.path,
        )

    def controllers(self) -> _controllers.ControllersClient:
        return _controllers.ControllersClient(
            self.client,
            self.path,
        )

    def ipams(self) -> _ipams.IpamsClient:
        return _ipams.IpamsClient(
            self.client,
            self.path,
        )

    def dns(self) -> _dns.DnsClient:
        return _dns.DnsClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def put(self) -> str:
        """
        Apply sdn controller changes && reload.
        """
        return self.client.put(self.path, parse_as=str)


@dataclass
class AsyncSdnClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'sdn'}"

    def vnets(self) -> _vnets.AsyncVnetsClient:
        return _vnets.AsyncVnetsClient(
            self.client,
            self.path,
        )

    def zones(self) -> _zones.AsyncZonesClient:
        return _zones.AsyncZonesClient(
            self.client,
            self.path,
        )

    def controllers(self) -> _controllers.AsyncControllersClient:
        return _controllers.AsyncControllersClient(
            self.client,
            self.path,
        )

    def ipams(self) -> _ipams.AsyncIpamsClient:
        return _ipams.AsyncIpamsClient(
            self.client,
            self.path,
        )

    def dns(self) -> _dns.AsyncDnsClient:
        return _dns.AsyncDnsClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def put(self) -> str:
        """
        Apply sdn controller changes && reload.
        """
        return await self.client.put(self.path, parse_as=str)
