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
    # The URL to download the file from.
    url: str
    # If false, no SSL/TLS certificates will be verified.
    verify_certificates: Optional[bool] = Field(
        alias="verify-certificates", default=None
    )

    class Config(CommonPydanticConfig):
        pass


@dataclass
class DownloadUrlClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'download-url'}"

    def post(self, parameters: PostParameters) -> str:
        """
        Download templates and ISO images by using an URL.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncDownloadUrlClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'download-url'}"

    async def post(self, parameters: PostParameters) -> str:
        """
        Download templates and ISO images by using an URL.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
