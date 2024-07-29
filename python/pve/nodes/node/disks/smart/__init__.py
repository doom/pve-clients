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
    attributes: Optional[list[dict[str, Any]]] = Field(default=None)
    health: str
    text: Optional[str] = Field(default=None)
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Block device name
    disk: str
    # If true returns only the health status
    healthonly: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class SmartClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'smart'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get SMART Health of a disk.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncSmartClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'smart'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get SMART Health of a disk.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
