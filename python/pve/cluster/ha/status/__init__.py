from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import current as _current
from . import manager_status as _manager_status


class GetResponseItem(BaseModel):
    pass


@dataclass
class StatusClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'status'}"

    def current(self) -> _current.CurrentClient:
        return _current.CurrentClient(
            self.client,
            self.path,
        )

    def manager_status(self) -> _manager_status.ManagerStatusClient:
        return _manager_status.ManagerStatusClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncStatusClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'status'}"

    def current(self) -> _current.AsyncCurrentClient:
        return _current.AsyncCurrentClient(
            self.client,
            self.path,
        )

    def manager_status(self) -> _manager_status.AsyncManagerStatusClient:
        return _manager_status.AsyncManagerStatusClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Directory index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
