from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class PutParameters(BaseModel):
    # Author of the mail
    author: Optional[str] = Field(default=None)
    # Comment
    comment: Optional[str] = Field(default=None)
    # A list of settings you want to delete.
    delete: Optional[list[str]] = Field(default=None)
    # Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Disable this target
    disable: Optional[bool] = Field(default=None)
    # `From` address for the mail
    from_address: Optional[str] = Field(alias="from-address", default=None)
    # List of email recipients
    mailto: Optional[list[str]] = Field(default=None)
    # List of users
    mailto_user: Optional[list[str]] = Field(alias="mailto-user", default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Author of the mail
    author: Optional[str] = Field(default=None)
    # Comment
    comment: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Disable this target
    disable: Optional[bool] = Field(default=None)
    # `From` address for the mail
    from_address: Optional[str] = Field(alias="from-address", default=None)
    # List of email recipients
    mailto: Optional[list[str]] = Field(default=None)
    # List of users
    mailto_user: Optional[list[str]] = Field(alias="mailto-user", default=None)
    # The name of the endpoint.
    name: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class NameClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    def delete(self):
        """
        Remove sendmail endpoint
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> GetResponseItem:
        """
        Return a specific sendmail endpoint
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update existing sendmail endpoint
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncNameClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    async def delete(self):
        """
        Remove sendmail endpoint
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> GetResponseItem:
        """
        Return a specific sendmail endpoint
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update existing sendmail endpoint
        """
        return await self.client.put(self.path, parameters)
