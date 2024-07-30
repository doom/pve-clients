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
    # Status.
    status: Optional[str] = Field(default=None)
    # Status details
    statusmsg: Optional[str] = Field(default=None)
    # Vnet identifier.
    vnet: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ContentClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'content'}"

    def get(self) -> list[GetResponseItem]:
        """
        List zone content.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncContentClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'content'}"

    async def get(self) -> list[GetResponseItem]:
        """
        List zone content.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
