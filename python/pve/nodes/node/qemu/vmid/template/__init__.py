from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # If you want to convert only 1 disk to base image.
    disk: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class TemplateClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'template'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Create a Template.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncTemplateClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'template'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Create a Template.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
