from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetResponseItem(BaseModel):
    # The current installed Proxmox VE Release
    release: str
    # The short git commit hash ID from which this version was build
    repoid: str
    # The current installed pve-manager package version
    version: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class VersionClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'version'}"

    def get(self) -> GetResponseItem:
        """
        API version details
        """
        return self.client.get(self.path, parse_as=GetResponseItem)


@dataclass
class AsyncVersionClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'version'}"

    async def get(self) -> GetResponseItem:
        """
        API version details
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)
