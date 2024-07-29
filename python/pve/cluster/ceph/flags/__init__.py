from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import flag as _flag


class PutParameters(BaseModel):
    # Backfilling of PGs is suspended.
    nobackfill: Optional[bool] = Field(default=None)
    # Deep Scrubbing is disabled.
    nodeep_scrub: Optional[bool] = Field(alias="nodeep-scrub", default=None)
    # OSD failure reports are being ignored, such that the monitors will not mark OSDs down.
    nodown: Optional[bool] = Field(default=None)
    # OSDs that were previously marked out will not be marked back in when they start.
    noin: Optional[bool] = Field(default=None)
    # OSDs will not automatically be marked out after the configured interval.
    noout: Optional[bool] = Field(default=None)
    # Rebalancing of PGs is suspended.
    norebalance: Optional[bool] = Field(default=None)
    # Recovery of PGs is suspended.
    norecover: Optional[bool] = Field(default=None)
    # Scrubbing is disabled.
    noscrub: Optional[bool] = Field(default=None)
    # Cache tiering activity is suspended.
    notieragent: Optional[bool] = Field(default=None)
    # OSDs are not allowed to start.
    noup: Optional[bool] = Field(default=None)
    # Pauses read and writes.
    pause: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Flag description.
    description: str
    # Flag name.
    name: str
    # Flag value.
    value: bool

    class Config(CommonPydanticConfig):
        pass


@dataclass
class FlagsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'flags'}"

    def flag(self, flag: str) -> _flag.FlagClient:
        return _flag.FlagClient(
            self.client,
            self.path,
            flag,
        )

    def get(self) -> list[GetResponseItem]:
        """
        get the status of all ceph flags
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def put(self, parameters: PutParameters) -> str:
        """
        Set/Unset multiple ceph flags at once.
        """
        return self.client.put(self.path, parameters, parse_as=str)


@dataclass
class AsyncFlagsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'flags'}"

    def flag(self, flag: str) -> _flag.AsyncFlagClient:
        return _flag.AsyncFlagClient(
            self.client,
            self.path,
            flag,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        get the status of all ceph flags
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def put(self, parameters: PutParameters) -> str:
        """
        Set/Unset multiple ceph flags at once.
        """
        return await self.client.put(self.path, parameters, parse_as=str)
