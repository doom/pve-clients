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
    """
    Returns an object with a single `result` property.
    """

    pass


@dataclass
class FsfreezeFreezeClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'fsfreeze-freeze'}"

    def post(self) -> PostResponseItem:
        """
        Execute fsfreeze-freeze.
        """
        return self.client.post(self.path, parse_as=PostResponseItem)


@dataclass
class AsyncFsfreezeFreezeClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'fsfreeze-freeze'}"

    async def post(self) -> PostResponseItem:
        """
        Execute fsfreeze-freeze.
        """
        return await self.client.post(self.path, parse_as=PostResponseItem)
