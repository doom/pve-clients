from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import name as _name


class GetResponseItem(BaseModel):
    # Comment
    comment: Optional[str] = Field(default=None)
    # Show if this target is disabled
    disable: Optional[bool] = Field(default=None)
    # Name of the target.
    name: str
    # Show if this entry was created by a user or was built-in
    origin: str
    # Type of the target.
    type: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class TargetsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'targets'}"

    def name(self, name: str) -> _name.NameClient:
        return _name.NameClient(
            self.client,
            self.path,
            name,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Returns a list of all entities that can be used as notification targets.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncTargetsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'targets'}"

    def name(self, name: str) -> _name.AsyncNameClient:
        return _name.AsyncNameClient(
            self.client,
            self.path,
            name,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Returns a list of all entities that can be used as notification targets.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
