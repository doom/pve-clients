from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import not_backed_up as _not_backed_up


class GetResponseItem(BaseModel):
    # API sub-directory endpoint
    subdir: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class BackupInfoClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'backup-info'}"

    def not_backed_up(self) -> _not_backed_up.NotBackedUpClient:
        return _not_backed_up.NotBackedUpClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Index for backup info related endpoints
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncBackupInfoClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'backup-info'}"

    def not_backed_up(self) -> _not_backed_up.AsyncNotBackedUpClient:
        return _not_backed_up.AsyncNotBackedUpClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Index for backup info related endpoints
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
