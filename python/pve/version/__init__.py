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
    # The default console viewer to use.
    console: Optional[str] = Field(default=None)
    # The current Proxmox VE point release in `x.y` format.
    release: str
    # The short git revision from which this version was build.
    repoid: str
    # The full pve-manager package version of this node.
    version: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class VersionClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient):
        self.client = client
        self.path = "version"

    def get(self) -> GetResponseItem:
        """
        API version details, including some parts of the global datacenter config.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)


@dataclass
class AsyncVersionClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient):
        self.client = client
        self.path = "version"

    async def get(self) -> GetResponseItem:
        """
        API version details, including some parts of the global datacenter config.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)
