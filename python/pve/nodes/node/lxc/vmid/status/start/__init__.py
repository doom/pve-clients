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
    # If set, enables very verbose debug log-level on start.
    debug: Optional[bool] = Field(default=None)
    # Ignore locks - only root is allowed to use this option.
    skiplock: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class StartClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'start'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Start the container.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncStartClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'start'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Start the container.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
