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
    # Whether the tasklog file should be downloaded. This parameter can't be used in conjunction with other parameters
    download: Optional[bool] = Field(default=None)
    # The amount of lines to read from the tasklog.
    limit: Optional[int] = Field(default=None)
    # Start at this line when reading the tasklog
    start: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class LogClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'log'}"

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Read task log.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncLogClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'log'}"

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Read task log.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
