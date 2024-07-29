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
    # The expected checksum of the file.
    checksum: Optional[str] = Field(default=None)
    # The algorithm to calculate the checksum of the file.
    checksum_algorithm: Optional[str] = Field(alias="checksum-algorithm", default=None)
    # Content type.
    content: str
    # The name of the file to create. Caution: This will be normalized!
    filename: str
    # The source file name. This parameter is usually set by the REST handler. You can only overwrite it when connecting to the trusted port on localhost.
    tmpfilename: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class UploadClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'upload'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Upload templates and ISO images.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncUploadClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'upload'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Upload templates and ISO images.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
