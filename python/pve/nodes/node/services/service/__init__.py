from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import reload as _reload
from . import restart as _restart
from . import start as _start
from . import state as _state
from . import stop as _stop


class GetResponseItem(BaseModel):
    subdir: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ServiceClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, service: str):
        self.client = client
        self.path = f"{parent_path}/{service}"

    def state(self) -> _state.StateClient:
        return _state.StateClient(
            self.client,
            self.path,
        )

    def start(self) -> _start.StartClient:
        return _start.StartClient(
            self.client,
            self.path,
        )

    def stop(self) -> _stop.StopClient:
        return _stop.StopClient(
            self.client,
            self.path,
        )

    def restart(self) -> _restart.RestartClient:
        return _restart.RestartClient(
            self.client,
            self.path,
        )

    def reload(self) -> _reload.ReloadClient:
        return _reload.ReloadClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Directory index
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncServiceClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, service: str):
        self.client = client
        self.path = f"{parent_path}/{service}"

    def state(self) -> _state.AsyncStateClient:
        return _state.AsyncStateClient(
            self.client,
            self.path,
        )

    def start(self) -> _start.AsyncStartClient:
        return _start.AsyncStartClient(
            self.client,
            self.path,
        )

    def stop(self) -> _stop.AsyncStopClient:
        return _stop.AsyncStopClient(
            self.client,
            self.path,
        )

    def restart(self) -> _restart.AsyncRestartClient:
        return _restart.AsyncRestartClient(
            self.client,
            self.path,
        )

    def reload(self) -> _reload.AsyncReloadClient:
        return _reload.AsyncReloadClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Directory index
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
