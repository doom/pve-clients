from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # Ignore locks - only root is allowed to use this option.
    skiplock: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ResetClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'reset'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Reset virtual machine.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncResetClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'reset'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Reset virtual machine.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
