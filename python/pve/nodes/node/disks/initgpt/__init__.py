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
    # Block device name
    disk: str
    # UUID for the GPT table
    uuid: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class InitgptClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'initgpt'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Initialize Disk with GPT
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncInitgptClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'initgpt'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Initialize Disk with GPT
        """
        return await self.client.post(self.path, parameters, parse_as=str)
