from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import name as _name


class PostParameters(BaseModel):
    # Author of the mail. Defaults to 'Proxmox VE'.
    author: Optional[str] = Field(default=None)
    # Comment
    comment: Optional[str] = Field(default=None)
    # Disable this target
    disable: Optional[bool] = Field(default=None)
    # `From` address for the mail
    from_address: str = Field(alias="from-address")
    # List of email recipients
    mailto: Optional[list[str]] = Field(default=None)
    # List of users
    mailto_user: Optional[list[str]] = Field(alias="mailto-user", default=None)
    # Determine which encryption method shall be used for the connection.
    mode: Optional[str] = Field(default=None)
    # The name of the endpoint.
    name: str
    # Password for SMTP authentication
    password: Optional[str] = Field(default=None)
    # The port to be used. Defaults to 465 for TLS based connections, 587 for STARTTLS based connections and port 25 for insecure plain-text connections.
    port: Optional[int] = Field(default=None)
    # The address of the SMTP server.
    server: str
    # Username for SMTP authentication
    username: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Author of the mail. Defaults to 'Proxmox VE'.
    author: Optional[str] = Field(default=None)
    # Comment
    comment: Optional[str] = Field(default=None)
    # Disable this target
    disable: Optional[bool] = Field(default=None)
    # `From` address for the mail
    from_address: str = Field(alias="from-address")
    # List of email recipients
    mailto: Optional[list[str]] = Field(default=None)
    # List of users
    mailto_user: Optional[list[str]] = Field(alias="mailto-user", default=None)
    # Determine which encryption method shall be used for the connection.
    mode: Optional[str] = Field(default=None)
    # The name of the endpoint.
    name: str
    # Show if this entry was created by a user or was built-in
    origin: str
    # The port to be used. Defaults to 465 for TLS based connections, 587 for STARTTLS based connections and port 25 for insecure plain-text connections.
    port: Optional[int] = Field(default=None)
    # The address of the SMTP server.
    server: str
    # Username for SMTP authentication
    username: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class SmtpClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'smtp'}"

    def name(self, name: str) -> _name.NameClient:
        return _name.NameClient(
            self.client,
            self.path,
            name,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Returns a list of all smtp endpoints
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create a new smtp endpoint
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncSmtpClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'smtp'}"

    def name(self, name: str) -> _name.AsyncNameClient:
        return _name.AsyncNameClient(
            self.client,
            self.path,
            name,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Returns a list of all smtp endpoints
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters):
        """
        Create a new smtp endpoint
        """
        return await self.client.post(self.path, parameters)
