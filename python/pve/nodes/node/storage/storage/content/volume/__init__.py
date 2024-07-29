from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PutParameters(BaseModel):
    # The new notes.
    notes: Optional[str] = Field(default=None)
    # Protection status. Currently only supported for backups.
    protected: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # Target volume identifier
    target: str
    # Target node. Default is local node.
    target_node: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Format identifier ('raw', 'qcow2', 'subvol', 'iso', 'tgz' ...)
    format: str
    # Optional notes.
    notes: Optional[str] = Field(default=None)
    # The Path
    path: str
    # Protection status. Currently only supported for backups.
    protected: Optional[bool] = Field(default=None)
    # Volume size in bytes.
    size: int
    # Used space. Please note that most storage plugins do not report anything useful here.
    used: int

    class Config(CommonPydanticConfig):
        pass


class DeleteParameters(BaseModel):
    # Time to wait for the task to finish. We return 'null' if the task finish within that time.
    delay: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class VolumeClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, volume: str):
        self.client = client
        self.path = f"{parent_path}/{volume}"

    def delete(self, parameters: DeleteParameters) -> Optional[str]:
        """
        Delete volume
        """
        return self.client.delete(self.path, parameters, parse_as=Optional[str])

    def get(self) -> GetResponseItem:
        """
        Get volume attributes
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def post(self, parameters: PostParameters) -> str:
        """
        Copy a volume. This is experimental code - do not use.
        """
        return self.client.post(self.path, parameters, parse_as=str)

    def put(self, parameters: PutParameters):
        """
        Update volume attributes
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncVolumeClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, volume: str):
        self.client = client
        self.path = f"{parent_path}/{volume}"

    async def delete(self, parameters: DeleteParameters) -> Optional[str]:
        """
        Delete volume
        """
        return await self.client.delete(self.path, parameters, parse_as=Optional[str])

    async def get(self) -> GetResponseItem:
        """
        Get volume attributes
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def post(self, parameters: PostParameters) -> str:
        """
        Copy a volume. This is experimental code - do not use.
        """
        return await self.client.post(self.path, parameters, parse_as=str)

    async def put(self, parameters: PutParameters):
        """
        Update volume attributes
        """
        return await self.client.put(self.path, parameters)
