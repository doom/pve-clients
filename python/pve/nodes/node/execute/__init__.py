from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PostResponseItem(BaseModel):
    pass


class PostParameters(BaseModel):
    # JSON encoded array of commands.
    commands: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ExecuteClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'execute'}"

    def post(self, parameters: PostParameters) -> list[PostResponseItem]:
        """
        Execute multiple commands in order, root only.
        """
        return self.client.post(self.path, parameters, parse_as=list[PostResponseItem])


@dataclass
class AsyncExecuteClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'execute'}"

    async def post(self, parameters: PostParameters) -> list[PostResponseItem]:
        """
        Execute multiple commands in order, root only.
        """
        return await self.client.post(
            self.path, parameters, parse_as=list[PostResponseItem]
        )
