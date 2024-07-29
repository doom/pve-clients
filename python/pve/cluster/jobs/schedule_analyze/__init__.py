from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetResponseItem(BaseModel):
    # UNIX timestamp for the run.
    timestamp: int
    # UTC timestamp for the run.
    utc: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Number of event-iteration to simulate and return.
    iterations: Optional[int] = Field(default=None)
    # Job schedule. The format is a subset of `systemd` calendar events.
    schedule: str
    # UNIX timestamp to start the calculation from. Defaults to the current time.
    starttime: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ScheduleAnalyzeClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'schedule-analyze'}"

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Returns a list of future schedule runtimes.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncScheduleAnalyzeClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'schedule-analyze'}"

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Returns a list of future schedule runtimes.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
