from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # Ignore locks - only root is allowed to use this option.
    skiplock: Optional[bool] = Field(default=None)
    # The storage for the VM state
    statestorage: Optional[str] = Field(default=None)
    # If set, suspends the VM to disk. Will be resumed on next VM start.
    todisk: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class SuspendClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'suspend'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Suspend virtual machine.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncSuspendClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'suspend'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Suspend virtual machine.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
