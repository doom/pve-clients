from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # Ceph service name.
    service: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class StartClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'start'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Start ceph services.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncStartClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'start'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Start ceph services.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
