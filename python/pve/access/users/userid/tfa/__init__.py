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
    # The type of TFA the users realm has set, if any.
    realm: Optional[str] = Field(default=None)
    # Array of the user configured TFA types, if any. Only available if 'multiple' was not passed.
    types: Optional[list[str]] = Field(default=None)
    # The type of TFA the user has set, if any. Only set if 'multiple' was not passed.
    user: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Request all entries as an array.
    multiple: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class TfaClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'tfa'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get user TFA types (Personal and Realm).
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncTfaClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'tfa'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get user TFA types (Personal and Realm).
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
