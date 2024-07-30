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
class UnlockTfaClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'unlock-tfa'}"

    def put(self) -> bool:
        """
        Unlock a user's TFA authentication.
        """
        return self.client.put(self.path, parse_as=bool)


@dataclass
class AsyncUnlockTfaClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'unlock-tfa'}"

    async def put(self) -> bool:
        """
        Unlock a user's TFA authentication.
        """
        return await self.client.put(self.path, parse_as=bool)
