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
    # If set, instructs a deep scrub instead of a normal one.
    deep: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ScrubClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'scrub'}"

    def post(self, parameters: PostParameters):
        """
        Instruct the OSD to scrub.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncScrubClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'scrub'}"

    async def post(self, parameters: PostParameters):
        """
        Instruct the OSD to scrub.
        """
        return await self.client.post(self.path, parameters)
