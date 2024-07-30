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
class TestClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'test'}"

    def post(self):
        """
        Send a test notification to a provided target.
        """
        return self.client.post(
            self.path,
        )


@dataclass
class AsyncTestClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'test'}"

    async def post(self):
        """
        Send a test notification to a provided target.
        """
        return await self.client.post(
            self.path,
        )
