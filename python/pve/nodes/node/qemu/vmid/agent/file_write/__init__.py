from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PostParameters(BaseModel):
    # The content to write into the file.
    content: str
    # If set, the content will be encoded as base64 (required by QEMU).Otherwise the content needs to be encoded beforehand - defaults to true.
    encode: Optional[bool] = Field(default=None)
    # The path to the file.
    file: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class FileWriteClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'file-write'}"

    def post(self, parameters: PostParameters):
        """
        Writes the given file via guest agent.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncFileWriteClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'file-write'}"

    async def post(self, parameters: PostParameters):
        """
        Writes the given file via guest agent.
        """
        return await self.client.post(self.path, parameters)
