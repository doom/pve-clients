from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PutParameters(BaseModel):
    # The key (qemu monitor encoding).
    key: str
    # Ignore locks - only root is allowed to use this option.
    skiplock: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class SendkeyClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'sendkey'}"

    def put(self, parameters: PutParameters):
        """
        Send key event to virtual machine.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncSendkeyClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'sendkey'}"

    async def put(self, parameters: PutParameters):
        """
        Send key event to virtual machine.
        """
        return await self.client.put(self.path, parameters)
