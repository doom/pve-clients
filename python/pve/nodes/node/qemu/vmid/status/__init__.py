from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import current as _current
from . import reboot as _reboot
from . import reset as _reset
from . import resume as _resume
from . import shutdown as _shutdown
from . import start as _start
from . import stop as _stop
from . import suspend as _suspend


class GetResponseItem(BaseModel):
    subdir: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class StatusClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'status'}"

    def current(self) -> _current.CurrentClient:
        return _current.CurrentClient(
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

    def reset(self) -> _reset.ResetClient:
        return _reset.ResetClient(
            self.client,
            self.path,
        )

    def shutdown(self) -> _shutdown.ShutdownClient:
        return _shutdown.ShutdownClient(
            self.client,
            self.path,
        )

    def reboot(self) -> _reboot.RebootClient:
        return _reboot.RebootClient(
            self.client,
            self.path,
        )

    def suspend(self) -> _suspend.SuspendClient:
        return _suspend.SuspendClient(
            self.client,
            self.path,
        )

    def resume(self) -> _resume.ResumeClient:
        return _resume.ResumeClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Directory index
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncStatusClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'status'}"

    def current(self) -> _current.AsyncCurrentClient:
        return _current.AsyncCurrentClient(
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

    def reset(self) -> _reset.AsyncResetClient:
        return _reset.AsyncResetClient(
            self.client,
            self.path,
        )

    def shutdown(self) -> _shutdown.AsyncShutdownClient:
        return _shutdown.AsyncShutdownClient(
            self.client,
            self.path,
        )

    def reboot(self) -> _reboot.AsyncRebootClient:
        return _reboot.AsyncRebootClient(
            self.client,
            self.path,
        )

    def suspend(self) -> _suspend.AsyncSuspendClient:
        return _suspend.AsyncSuspendClient(
            self.client,
            self.path,
        )

    def resume(self) -> _resume.AsyncResumeClient:
        return _resume.AsyncResumeClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Directory index
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
