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
    nocheck: Optional[bool] = Field(default=None)
    # Ignore locks - only root is allowed to use this option.
    skiplock: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ResumeClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'resume'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Resume virtual machine.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncResumeClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'resume'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Resume virtual machine.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
