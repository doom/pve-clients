from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PostParameters(BaseModel):
    # The monitor command.
    command: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MonitorClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'monitor'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Execute QEMU monitor commands.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncMonitorClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'monitor'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Execute QEMU monitor commands.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
