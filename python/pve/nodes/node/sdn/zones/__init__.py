from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import zone as _zone


class GetResponseItem(BaseModel):
    # Status of zone
    status: str
    # The SDN zone object identifier.
    zone: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ZonesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'zones'}"

    def zone(self, zone: str) -> _zone.ZoneClient:
        return _zone.ZoneClient(
            self.client,
            self.path,
            zone,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Get status for all zones.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncZonesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'zones'}"

    def zone(self, zone: str) -> _zone.AsyncZoneClient:
        return _zone.AsyncZoneClient(
            self.client,
            self.path,
            zone,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Get status for all zones.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
