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
    pass


@dataclass
class StateClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'state'}"

    def get(self) -> GetResponseItem:
        """
        Read service properties
        """
        return self.client.get(self.path, parse_as=GetResponseItem)


@dataclass
class AsyncStateClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'state'}"

    async def get(self) -> GetResponseItem:
        """
        Read service properties
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)
