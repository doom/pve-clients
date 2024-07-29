from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetResponseItem(BaseModel):
    pass


@dataclass
class ManagerStatusClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'manager_status'}"

    def get(self) -> GetResponseItem:
        """
        Get full HA manger status, including LRM status.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)


@dataclass
class AsyncManagerStatusClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'manager_status'}"

    async def get(self) -> GetResponseItem:
        """
        Get full HA manger status, including LRM status.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)
