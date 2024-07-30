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
    # Send notification about new packages.
    notify: Optional[bool] = Field(default=None)
    # Only produces output suitable for logging, omitting progress indicators.
    quiet: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


@dataclass
class UpdateClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'update'}"

    def get(self) -> list[GetResponseItem]:
        """
        List available updates.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters) -> str:
        """
        This is used to resynchronize the package index files from their sources (apt-get update).
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncUpdateClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'update'}"

    async def get(self) -> list[GetResponseItem]:
        """
        List available updates.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters) -> str:
        """
        This is used to resynchronize the package index files from their sources (apt-get update).
        """
        return await self.client.post(self.path, parameters, parse_as=str)
