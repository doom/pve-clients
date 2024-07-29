from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PutResponseItem(BaseModel):
    """
    Updated token information.
    """

    comment: Optional[str] = Field(default=None)
    # API token expiration date (seconds since epoch). '0' means no expiration date.
    expire: Optional[int] = Field(default=None)
    # Restrict API token privileges with separate ACLs (default), or give full privileges of corresponding user.
    privsep: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class PutParameters(BaseModel):
    comment: Optional[str] = Field(default=None)
    # API token expiration date (seconds since epoch). '0' means no expiration date.
    expire: Optional[int] = Field(default=None)
    # Restrict API token privileges with separate ACLs (default), or give full privileges of corresponding user.
    privsep: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class Info(BaseModel):
    comment: Optional[str] = Field(default=None)
    # API token expiration date (seconds since epoch). '0' means no expiration date.
    expire: Optional[int] = Field(default=None)
    # Restrict API token privileges with separate ACLs (default), or give full privileges of corresponding user.
    privsep: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class PostResponseItem(BaseModel):
    # The full token id.
    full_tokenid: str = Field(alias="full-tokenid")
    info: Info
    # API token value used for authentication.
    value: str

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    comment: Optional[str] = Field(default=None)
    # API token expiration date (seconds since epoch). '0' means no expiration date.
    expire: Optional[int] = Field(default=None)
    # Restrict API token privileges with separate ACLs (default), or give full privileges of corresponding user.
    privsep: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    comment: Optional[str] = Field(default=None)
    # API token expiration date (seconds since epoch). '0' means no expiration date.
    expire: Optional[int] = Field(default=None)
    # Restrict API token privileges with separate ACLs (default), or give full privileges of corresponding user.
    privsep: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class TokenidClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, tokenid: str):
        self.client = client
        self.path = f"{parent_path}/{tokenid}"

    def delete(self):
        """
        Remove API token for a specific user.
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> GetResponseItem:
        """
        Get specific API token information.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Generate a new API token for a specific user. NOTE: returns API token value, which needs to be stored as it cannot be retrieved afterwards!
        """
        return self.client.post(self.path, parameters, parse_as=PostResponseItem)

    def put(self, parameters: PutParameters) -> PutResponseItem:
        """
        Update API token for a specific user.
        """
        return self.client.put(self.path, parameters, parse_as=PutResponseItem)


@dataclass
class AsyncTokenidClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, tokenid: str):
        self.client = client
        self.path = f"{parent_path}/{tokenid}"

    async def delete(self):
        """
        Remove API token for a specific user.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> GetResponseItem:
        """
        Get specific API token information.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Generate a new API token for a specific user. NOTE: returns API token value, which needs to be stored as it cannot be retrieved afterwards!
        """
        return await self.client.post(self.path, parameters, parse_as=PostResponseItem)

    async def put(self, parameters: PutParameters) -> PutResponseItem:
        """
        Update API token for a specific user.
        """
        return await self.client.put(self.path, parameters, parse_as=PutResponseItem)
