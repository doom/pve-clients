from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


@dataclass
class WakeonlanClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'wakeonlan'}"

    def post(self) -> str:
        """
        Try to wake a node via 'wake on LAN' network packet.
        """
        return self.client.post(self.path, parse_as=str)


@dataclass
class AsyncWakeonlanClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'wakeonlan'}"

    async def post(self) -> str:
        """
        Try to wake a node via 'wake on LAN' network packet.
        """
        return await self.client.post(self.path, parse_as=str)
