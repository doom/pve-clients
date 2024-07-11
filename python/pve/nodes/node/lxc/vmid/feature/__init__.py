from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    has_feature: bool = Field(alias="hasFeature")

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Feature to check.
    feature: str
    # The name of the snapshot.
    snapname: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class FeatureClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'feature'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Check if feature for virtual machine is available.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncFeatureClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'feature'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Check if feature for virtual machine is available.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
