from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import cpu as _cpu
from . import machines as _machines


class GetResponseItem(BaseModel):
    pass


@dataclass
class QemuClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'qemu'}"

    def cpu(self) -> _cpu.CpuClient:
        return _cpu.CpuClient(
            self.client,
            self.path,
        )

    def machines(self) -> _machines.MachinesClient:
        return _machines.MachinesClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        QEMU capabilities index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncQemuClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'qemu'}"

    def cpu(self) -> _cpu.AsyncCpuClient:
        return _cpu.AsyncCpuClient(
            self.client,
            self.path,
        )

    def machines(self) -> _machines.AsyncMachinesClient:
        return _machines.AsyncMachinesClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        QEMU capabilities index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
