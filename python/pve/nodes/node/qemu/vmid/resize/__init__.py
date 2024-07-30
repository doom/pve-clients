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
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # The disk you want to resize.
    disk: str
    # The new size. With the `+` sign the value is added to the actual size of the volume and without it, the value is taken as an absolute one. Shrinking disk size is not supported.
    size: str
    # Ignore locks - only root is allowed to use this option.
    skiplock: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ResizeClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'resize'}"

    def put(self, parameters: PutParameters) -> str:
        """
        Extend volume size.
        """
        return self.client.put(self.path, parameters, parse_as=str)


@dataclass
class AsyncResizeClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'resize'}"

    async def put(self, parameters: PutParameters) -> str:
        """
        Extend volume size.
        """
        return await self.client.put(self.path, parameters, parse_as=str)
