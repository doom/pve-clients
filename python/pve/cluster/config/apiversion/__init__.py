from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


@dataclass
class ApiversionClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'apiversion'}"

    def get(self) -> int:
        """
        Return the version of the cluster join API available on this node.
        """
        return self.client.get(self.path, parse_as=int)


@dataclass
class AsyncApiversionClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'apiversion'}"

    async def get(self) -> int:
        """
        Return the version of the cluster join API available on this node.
        """
        return await self.client.get(self.path, parse_as=int)
