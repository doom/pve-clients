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
    # The new password.
    password: str
    # Full User ID, in the `name@realm` format.
    userid: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class PasswordClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'password'}"

    def put(self, parameters: PutParameters):
        """
        Change user password.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncPasswordClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'password'}"

    async def put(self, parameters: PutParameters):
        """
        Change user password.
        """
        return await self.client.put(self.path, parameters)
