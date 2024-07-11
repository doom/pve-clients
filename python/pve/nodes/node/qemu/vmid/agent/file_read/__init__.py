from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    """
    Returns an object with a `content` property.
    """

    # The content of the file, maximum 16777216
    content: str
    # If set to 1, the output is truncated and not complete
    truncated: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # The path to the file
    file: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class FileReadClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'file-read'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Reads the given file via guest agent. Is limited to 16777216 bytes.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncFileReadClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'file-read'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Reads the given file via guest agent. Is limited to 16777216 bytes.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
