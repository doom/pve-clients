from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PostResponseItem(BaseModel):
    # The PID of the process started by the guest-agent.
    pid: int

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # The command as a list of program + arguments.
    command: list[str]
    # Data to pass as 'input-data' to the guest. Usually treated as STDIN to 'command'.
    input_data: Optional[str] = Field(alias="input-data", default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ExecClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'exec'}"

    def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Executes the given command in the vm via the guest-agent and returns an object with the pid.
        """
        return self.client.post(self.path, parameters, parse_as=PostResponseItem)


@dataclass
class AsyncExecClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'exec'}"

    async def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Executes the given command in the vm via the guest-agent and returns an object with the pid.
        """
        return await self.client.post(self.path, parameters, parse_as=PostResponseItem)
