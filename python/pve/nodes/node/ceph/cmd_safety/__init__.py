from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    # If it is safe to run the command.
    safe: bool
    # Status message given by Ceph.
    status: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Action to check
    action: str
    # ID of the service
    id: str
    # Service type
    service: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class CmdSafetyClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'cmd-safety'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Heuristical check if it is safe to perform an action.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncCmdSafetyClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'cmd-safety'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Heuristical check if it is safe to perform an action.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
