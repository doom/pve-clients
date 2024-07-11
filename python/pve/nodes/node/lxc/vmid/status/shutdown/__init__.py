from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # Make sure the Container stops.
    force_stop: Optional[bool] = Field(alias="forceStop", default=None)
    # Wait maximal timeout seconds.
    timeout: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ShutdownClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'shutdown'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Shutdown the container. This will trigger a clean shutdown of the container, see lxc-stop(1) for details.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncShutdownClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'shutdown'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Shutdown the container. This will trigger a clean shutdown of the container, see lxc-stop(1) for details.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
