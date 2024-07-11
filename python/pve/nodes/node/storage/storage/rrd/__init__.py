from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    filename: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # The RRD consolidation function
    cf: Optional[str] = Field(default=None)
    # The list of datasources you want to display.
    ds: str
    # Specify the time frame you are interested in.
    timeframe: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class RrdClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'rrd'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Read storage RRD statistics (returns PNG).
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncRrdClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'rrd'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Read storage RRD statistics (returns PNG).
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
