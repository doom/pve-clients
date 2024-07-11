from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # Force a hard-stop after the timeout.
    force_stop: Optional[bool] = Field(alias="force-stop", default=None)
    # Timeout for each guest shutdown task. Depending on `force-stop`, the shutdown gets then simply aborted or a hard-stop is forced.
    timeout: Optional[int] = Field(default=None)
    # Only consider Guests with these IDs.
    vms: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class StopallClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'stopall'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Stop all VMs and Containers.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncStopallClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'stopall'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Stop all VMs and Containers.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
