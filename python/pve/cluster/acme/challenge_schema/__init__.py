from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class Schema(BaseModel):
    pass


class GetResponseItem(BaseModel):
    id: str
    # Human readable name, falls back to id
    name: str
    schema: Schema
    type: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ChallengeSchemaClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'challenge-schema'}"

    def get(self) -> list[GetResponseItem]:
        """
        Get schema of ACME challenge types.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncChallengeSchemaClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'challenge-schema'}"

    async def get(self) -> list[GetResponseItem]:
        """
        Get schema of ACME challenge types.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
