from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # The storage where the template will be stored
    storage: str
    # The template which will downloaded
    template: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


@dataclass
class AplinfoClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'aplinfo'}"

    def get(self) -> list[GetResponseItem]:
        """
        Get list of appliances.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters) -> str:
        """
        Download appliance templates.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncAplinfoClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'aplinfo'}"

    async def get(self) -> list[GetResponseItem]:
        """
        Get list of appliances.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters) -> str:
        """
        Download appliance templates.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
