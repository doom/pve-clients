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
    """
    Returns an object with a single `result` property.
    """

    pass


@dataclass
class GetVcpusClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'get-vcpus'}"

    def get(self) -> GetResponseItem:
        """
        Execute get-vcpus.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)


@dataclass
class AsyncGetVcpusClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'get-vcpus'}"

    async def get(self) -> GetResponseItem:
        """
        Execute get-vcpus.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)
