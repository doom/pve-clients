from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PutParameters(BaseModel):
    # Proxmox VE subscription key
    key: str

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # Always connect to server, even if we have up to date info inside local cache.
    force: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


@dataclass
class SubscriptionClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'subscription'}"

    def delete(self):
        """
        Delete subscription key of this node.
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> GetResponseItem:
        """
        Read subscription info.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def post(self, parameters: PostParameters):
        """
        Update subscription info.
        """
        return self.client.post(self.path, parameters)

    def put(self, parameters: PutParameters):
        """
        Set subscription key.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncSubscriptionClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'subscription'}"

    async def delete(self):
        """
        Delete subscription key of this node.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> GetResponseItem:
        """
        Read subscription info.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def post(self, parameters: PostParameters):
        """
        Update subscription info.
        """
        return await self.client.post(self.path, parameters)

    async def put(self, parameters: PutParameters):
        """
        Set subscription key.
        """
        return await self.client.put(self.path, parameters)
