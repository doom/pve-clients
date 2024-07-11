from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import service as _service


class GetResponseItem(BaseModel):
    pass


@dataclass
class ServicesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'services'}"

    def service(self, service: str) -> _service.ServiceClient:
        return _service.ServiceClient(
            self.client,
            self.path,
            service,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Service list.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncServicesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'services'}"

    def service(self, service: str) -> _service.AsyncServiceClient:
        return _service.AsyncServiceClient(
            self.client,
            self.path,
            service,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Service list.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
