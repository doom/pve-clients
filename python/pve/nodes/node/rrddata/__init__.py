from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    pass


class GetParameters(BaseModel):
    # The RRD consolidation function
    cf: Optional[str] = Field(default=None)
    # Specify the time frame you are interested in.
    timeframe: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class RrddataClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'rrddata'}"

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Read node RRD statistics
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncRrddataClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'rrddata'}"

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Read node RRD statistics
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
