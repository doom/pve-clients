from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetParameters(BaseModel):
    # base64-path to the directory or file to download.
    filepath: str
    # Backup volume ID or name. Currently only PBS snapshots are supported.
    volume: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class DownloadClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'download'}"

    def get(self, parameters: GetParameters) -> dict[str, Any]:
        """
        Extract a file or directory (as zip archive) from a PBS backup.
        """
        return self.client.get(self.path, parameters, parse_as=dict[str, Any])


@dataclass
class AsyncDownloadClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'download'}"

    async def get(self, parameters: GetParameters) -> dict[str, Any]:
        """
        Extract a file or directory (as zip archive) from a PBS backup.
        """
        return await self.client.get(self.path, parameters, parse_as=dict[str, Any])
