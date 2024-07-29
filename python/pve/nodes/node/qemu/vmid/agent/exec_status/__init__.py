from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetResponseItem(BaseModel):
    # stderr of the process
    err_data: Optional[str] = Field(alias="err-data", default=None)
    # true if stderr was not fully captured
    err_truncated: Optional[bool] = Field(alias="err-truncated", default=None)
    # process exit code if it was normally terminated.
    exitcode: Optional[int] = Field(default=None)
    # Tells if the given command has exited yet.
    exited: bool
    # stdout of the process
    out_data: Optional[str] = Field(alias="out-data", default=None)
    # true if stdout was not fully captured
    out_truncated: Optional[bool] = Field(alias="out-truncated", default=None)
    # signal number or exception code if the process was abnormally terminated.
    signal: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # The PID to query
    pid: int

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ExecStatusClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'exec-status'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Gets the status of the given pid started by the guest-agent
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncExecStatusClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'exec-status'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Gets the status of the given pid started by the guest-agent
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
