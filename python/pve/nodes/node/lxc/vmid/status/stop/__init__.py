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
    # Try to abort active 'vzshutdown' tasks before stopping.
    overrule_shutdown: Optional[bool] = Field(alias="overrule-shutdown", default=None)
    # Ignore locks - only root is allowed to use this option.
    skiplock: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class StopClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'stop'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Stop the container. This will abruptly stop all processes running in the container.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncStopClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'stop'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Stop the container. This will abruptly stop all processes running in the container.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
