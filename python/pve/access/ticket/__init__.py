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
    csrfprevention_token: Optional[str] = Field(
        alias="CSRFPreventionToken", default=None
    )
    clustername: Optional[str] = Field(default=None)
    ticket: Optional[str] = Field(default=None)
    username: str

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # With webauthn the format of half-authenticated tickts changed. New clients should pass 1 here and not worry about the old format. The old format is deprecated and will be retired with PVE-8.0
    new_format: Optional[bool] = Field(alias="new-format", default=None)
    # One-time password for Two-factor authentication.
    otp: Optional[str] = Field(default=None)
    # The secret password. This can also be a valid ticket.
    password: str
    # Verify ticket, and check if user have access 'privs' on 'path'
    path: Optional[str] = Field(default=None)
    # Verify ticket, and check if user have access 'privs' on 'path'
    privs: Optional[str] = Field(default=None)
    # You can optionally pass the realm using this parameter. Normally the realm is simply added to the username <username>@<relam>.
    realm: Optional[str] = Field(default=None)
    # The signed TFA challenge string the user wants to respond to.
    tfa_challenge: Optional[str] = Field(alias="tfa-challenge", default=None)
    # User name
    username: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class TicketClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'ticket'}"

    def get(self):
        """
        Dummy. Useful for formatters which want to provide a login page.
        """
        return self.client.get(
            self.path,
        )

    def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Create or verify authentication ticket.
        """
        return self.client.post(self.path, parameters, parse_as=PostResponseItem)


@dataclass
class AsyncTicketClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'ticket'}"

    async def get(self):
        """
        Dummy. Useful for formatters which want to provide a login page.
        """
        return await self.client.get(
            self.path,
        )

    async def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Create or verify authentication ticket.
        """
        return await self.client.post(self.path, parameters, parse_as=PostResponseItem)
