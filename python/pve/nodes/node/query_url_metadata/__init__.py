from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    filename: Optional[str] = Field(default=None)
    mimetype: Optional[str] = Field(default=None)
    size: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # The URL to query the metadata from.
    url: str
    # If false, no SSL/TLS certificates will be verified.
    verify_certificates: Optional[bool] = Field(
        alias="verify-certificates", default=None
    )

    class Config(CommonPydanticConfig):
        pass


@dataclass
class QueryUrlMetadataClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'query-url-metadata'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Query metadata of an URL: file size, file name and mime type.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncQueryUrlMetadataClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'query-url-metadata'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Query metadata of an URL: file size, file name and mime type.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
