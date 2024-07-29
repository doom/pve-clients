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
    # Issue start command even if virtual guest have 'onboot' not set or set to off.
    force: Optional[bool] = Field(default=None)
    # Only consider guests from this comma separated list of VMIDs.
    vms: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class StartallClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'startall'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Start all VMs and containers located on this node (by default only those with onboot=1).
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncStartallClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'startall'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Start all VMs and containers located on this node (by default only those with onboot=1).
        """
        return await self.client.post(self.path, parameters, parse_as=str)
