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
    # Line number
    n: int
    # Line text
    t: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    limit: Optional[int] = Field(default=None)
    # Service ID
    service: Optional[str] = Field(default=None)
    # Display all log since this date-time string.
    since: Optional[str] = Field(default=None)
    start: Optional[int] = Field(default=None)
    # Display all log until this date-time string.
    until: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class SyslogClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'syslog'}"

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Read system log
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncSyslogClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'syslog'}"

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Read system log
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
