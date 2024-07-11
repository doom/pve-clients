from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    # True if this is a custom CPU model.
    custom: bool
    # Name of the CPU model. Identifies it for subsequent API calls. Prefixed with 'custom-' for custom models.
    name: str
    # CPU vendor visible to the guest when this model is selected. Vendor of 'reported-model' in case of custom models.
    vendor: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class CpuClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'cpu'}"

    def get(self) -> list[GetResponseItem]:
        """
        List all custom and default CPU models.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncCpuClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'cpu'}"

    async def get(self) -> list[GetResponseItem]:
        """
        List all custom and default CPU models.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
