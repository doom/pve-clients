from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import dump as _dump


class GetResponseItem(BaseModel):
    # Configuration option name.
    key: str
    # The new pending value.
    new: Optional[str] = Field(default=None)
    # Value as it was used to generate the current cloudinit image.
    old: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class CloudinitClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'cloudinit'}"

    def dump(self) -> _dump.DumpClient:
        return _dump.DumpClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Get the cloudinit configuration with both current and pending values.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def put(self):
        """
        Regenerate and change cloudinit config drive.
        """
        return self.client.put(
            self.path,
        )


@dataclass
class AsyncCloudinitClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'cloudinit'}"

    def dump(self) -> _dump.AsyncDumpClient:
        return _dump.AsyncDumpClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Get the cloudinit configuration with both current and pending values.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def put(self):
        """
        Regenerate and change cloudinit config drive.
        """
        return await self.client.put(
            self.path,
        )