from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PutParameters(BaseModel):
    # Time zone. The file '/usr/share/zoneinfo/zone.tab' contains the list of valid names.
    timezone: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Seconds since 1970-01-01 00:00:00 (local time)
    localtime: int
    # Seconds since 1970-01-01 00:00:00 UTC.
    time: int
    # Time zone
    timezone: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class TimeClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'time'}"

    def get(self) -> GetResponseItem:
        """
        Read server time and time zone settings.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Set time zone.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncTimeClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'time'}"

    async def get(self) -> GetResponseItem:
        """
        Read server time and time zone settings.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Set time zone.
        """
        return await self.client.put(self.path, parameters)
