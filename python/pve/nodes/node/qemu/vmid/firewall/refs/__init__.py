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
    comment: Optional[str] = Field(default=None)
    name: str
    type: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Only list references of specified type.
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class RefsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'refs'}"

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Lists possible IPSet/Alias reference which are allowed in source/dest properties.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncRefsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'refs'}"

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Lists possible IPSet/Alias reference which are allowed in source/dest properties.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
