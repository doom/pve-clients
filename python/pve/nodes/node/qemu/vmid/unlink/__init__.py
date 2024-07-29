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
    # Force physical removal. Without this, we simple remove the disk from the config file and create an additional configuration entry called 'unused[n]', which contains the volume ID. Unlink of unused[n] always cause physical removal.
    force: Optional[bool] = Field(default=None)
    # A list of disk IDs you want to delete.
    idlist: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class UnlinkClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'unlink'}"

    def put(self, parameters: PutParameters):
        """
        Unlink/delete disk images.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncUnlinkClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'unlink'}"

    async def put(self, parameters: PutParameters):
        """
        Unlink/delete disk images.
        """
        return await self.client.put(self.path, parameters)
