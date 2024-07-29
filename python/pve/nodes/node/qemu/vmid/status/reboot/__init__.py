from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PostParameters(BaseModel):
    # Wait maximal timeout seconds for the shutdown.
    timeout: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class RebootClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'reboot'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Reboot the VM by shutting it down, and starting it again. Applies pending changes.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncRebootClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'reboot'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Reboot the VM by shutting it down, and starting it again. Applies pending changes.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
