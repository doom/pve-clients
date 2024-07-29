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
    # The new value of the flag
    value: bool

    class Config(CommonPydanticConfig):
        pass


@dataclass
class FlagClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, flag: str):
        self.client = client
        self.path = f"{parent_path}/{flag}"

    def get(self) -> bool:
        """
        Get the status of a specific ceph flag.
        """
        return self.client.get(self.path, parse_as=bool)

    def put(self, parameters: PutParameters):
        """
        Set or clear (unset) a specific ceph flag
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncFlagClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, flag: str):
        self.client = client
        self.path = f"{parent_path}/{flag}"

    async def get(self) -> bool:
        """
        Get the status of a specific ceph flag.
        """
        return await self.client.get(self.path, parse_as=bool)

    async def put(self, parameters: PutParameters):
        """
        Set or clear (unset) a specific ceph flag
        """
        return await self.client.put(self.path, parameters)
