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
    # Target node.
    node: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class RelocateClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'relocate'}"

    def post(self, parameters: PostParameters):
        """
        Request resource relocatzion to another node. This stops the service on the old node, and restarts it on the target node.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncRelocateClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'relocate'}"

    async def post(self, parameters: PostParameters):
        """
        Request resource relocatzion to another node. This stops the service on the old node, and restarts it on the target node.
        """
        return await self.client.post(self.path, parameters)
