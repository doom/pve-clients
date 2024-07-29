from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import server as _server


class GetResponseItem(BaseModel):
    pass


@dataclass
class MetricsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'metrics'}"

    def server(self) -> _server.ServerClient:
        return _server.ServerClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Metrics index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncMetricsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'metrics'}"

    def server(self) -> _server.AsyncServerClient:
        return _server.AsyncServerClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Metrics index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
