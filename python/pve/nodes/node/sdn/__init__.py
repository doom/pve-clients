from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import zones as _zones


class GetResponseItem(BaseModel):
    pass


@dataclass
class SdnClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'sdn'}"

    def zones(self) -> _zones.ZonesClient:
        return _zones.ZonesClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        SDN index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncSdnClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'sdn'}"

    def zones(self) -> _zones.AsyncZonesClient:
        return _zones.AsyncZonesClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        SDN index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
