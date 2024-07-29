from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


@dataclass
class SuspendClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'suspend'}"

    def post(self) -> str:
        """
        Suspend the container. This is experimental.
        """
        return self.client.post(self.path, parse_as=str)


@dataclass
class AsyncSuspendClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'suspend'}"

    async def post(self) -> str:
        """
        Suspend the container. This is experimental.
        """
        return await self.client.post(self.path, parse_as=str)
