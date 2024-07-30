from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import gotify as _gotify
from . import sendmail as _sendmail
from . import smtp as _smtp


class GetResponseItem(BaseModel):
    pass


@dataclass
class EndpointsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'endpoints'}"

    def sendmail(self) -> _sendmail.SendmailClient:
        return _sendmail.SendmailClient(
            self.client,
            self.path,
        )

    def gotify(self) -> _gotify.GotifyClient:
        return _gotify.GotifyClient(
            self.client,
            self.path,
        )

    def smtp(self) -> _smtp.SmtpClient:
        return _smtp.SmtpClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Index for all available endpoint types.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncEndpointsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'endpoints'}"

    def sendmail(self) -> _sendmail.AsyncSendmailClient:
        return _sendmail.AsyncSendmailClient(
            self.client,
            self.path,
        )

    def gotify(self) -> _gotify.AsyncGotifyClient:
        return _gotify.AsyncGotifyClient(
            self.client,
            self.path,
        )

    def smtp(self) -> _smtp.AsyncSmtpClient:
        return _smtp.AsyncSmtpClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Index for all available endpoint types.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
