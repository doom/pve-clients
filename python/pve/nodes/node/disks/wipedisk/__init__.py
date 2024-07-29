from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PutParameters(BaseModel):
    # Block device name
    disk: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class WipediskClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'wipedisk'}"

    def put(self, parameters: PutParameters) -> str:
        """
        Wipe a disk or partition.
        """
        return self.client.put(self.path, parameters, parse_as=str)


@dataclass
class AsyncWipediskClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'wipedisk'}"

    async def put(self, parameters: PutParameters) -> str:
        """
        Wipe a disk or partition.
        """
        return await self.client.put(self.path, parameters, parse_as=str)
