from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import upid as _upid


class GetResponseItem(BaseModel):
    endtime: Optional[int] = Field(default=None)
    id: str
    node: str
    pid: int
    pstart: int
    starttime: int
    status: Optional[str] = Field(default=None)
    type: str
    upid: str
    user: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Only list tasks with a status of ERROR.
    errors: Optional[bool] = Field(default=None)
    # Only list this amount of tasks.
    limit: Optional[int] = Field(default=None)
    # Only list tasks since this UNIX epoch.
    since: Optional[int] = Field(default=None)
    # List archived, active or all tasks.
    source: Optional[str] = Field(default=None)
    # List tasks beginning from this offset.
    start: Optional[int] = Field(default=None)
    # List of Task States that should be returned.
    statusfilter: Optional[str] = Field(default=None)
    # Only list tasks of this type (e.g., vzstart, vzdump).
    typefilter: Optional[str] = Field(default=None)
    # Only list tasks until this UNIX epoch.
    until: Optional[int] = Field(default=None)
    # Only list tasks from this user.
    userfilter: Optional[str] = Field(default=None)
    # Only list tasks for this VM.
    vmid: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class TasksClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'tasks'}"

    def upid(self, upid: str) -> _upid.UpidClient:
        return _upid.UpidClient(
            self.client,
            self.path,
            upid,
        )

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Read task list for one node (finished tasks).
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncTasksClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'tasks'}"

    def upid(self, upid: str) -> _upid.AsyncUpidClient:
        return _upid.AsyncUpidClient(
            self.client,
            self.path,
            upid,
        )

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Read task list for one node (finished tasks).
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
