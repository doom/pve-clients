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
    # More verbose description (if available).
    descr: str
    # Macro name.
    macro: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MacrosClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'macros'}"

    def get(self) -> list[GetResponseItem]:
        """
        List available macros
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncMacrosClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'macros'}"

    async def get(self) -> list[GetResponseItem]:
        """
        List available macros
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
