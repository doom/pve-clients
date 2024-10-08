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
    # Ceph service name.
    service: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class RestartClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'restart'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Restart ceph services.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncRestartClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'restart'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Restart ceph services.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
