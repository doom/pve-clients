from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostParameters(BaseModel):
    # The target content of /etc/hosts.
    data: str
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # The content of /etc/hosts.
    data: str
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class HostsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'hosts'}"

    def get(self) -> GetResponseItem:
        """
        Get the content of /etc/hosts.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def post(self, parameters: PostParameters):
        """
        Write /etc/hosts.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncHostsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'hosts'}"

    async def get(self) -> GetResponseItem:
        """
        Get the content of /etc/hosts.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def post(self, parameters: PostParameters):
        """
        Write /etc/hosts.
        """
        return await self.client.post(self.path, parameters)
