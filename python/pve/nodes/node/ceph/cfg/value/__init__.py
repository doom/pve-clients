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
    """
    Contains {section}->{key} children with the values
    """

    pass


class GetParameters(BaseModel):
    # List of <section>:<config key> items.
    config_keys: str = Field(alias="config-keys")

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ValueClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'value'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get configured values from either the config file or config DB.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncValueClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'value'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get configured values from either the config file or config DB.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
