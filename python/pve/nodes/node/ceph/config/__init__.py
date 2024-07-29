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
class ConfigClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'config'}"

    def get(self) -> str:
        """
        Get the Ceph configuration file. Deprecated, please use `/nodes/{node}/ceph/cfg/raw.
        """
        return self.client.get(self.path, parse_as=str)


@dataclass
class AsyncConfigClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'config'}"

    async def get(self) -> str:
        """
        Get the Ceph configuration file. Deprecated, please use `/nodes/{node}/ceph/cfg/raw.
        """
        return await self.client.get(self.path, parse_as=str)
