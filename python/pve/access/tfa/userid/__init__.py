from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import id as _id


class PostResponseItem(BaseModel):
    # When adding u2f entries, this contains a challenge the user must respond to in order to finish the registration.
    challenge: Optional[str] = Field(default=None)
    # The id of a newly added TFA entry.
    id: str
    # When adding recovery codes, this contains the list of codes to be displayed to the user
    recovery: Optional[list[str]] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # When responding to a u2f challenge: the original challenge string
    challenge: Optional[str] = Field(default=None)
    # A description to distinguish multiple entries from one another
    description: Optional[str] = Field(default=None)
    # The current password of the user performing the change.
    password: Optional[str] = Field(default=None)
    # A totp URI.
    totp: Optional[str] = Field(default=None)
    # TFA Entry Type.
    type: str
    # The current value for the provided totp URI, or a Webauthn/U2F challenge response
    value: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    """
    TFA Entry.
    """

    # Creation time of this entry as unix epoch.
    created: int
    # User chosen description for this entry.
    description: str
    # Whether this TFA entry is currently enabled.
    enable: Optional[bool] = Field(default=None)
    # The id used to reference this entry.
    id: str
    # TFA Entry Type.
    type: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class UseridClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, userid: str):
        self.client = client
        self.path = f"{parent_path}/{userid}"

    def id(self, id: str) -> _id.IdClient:
        return _id.IdClient(
            self.client,
            self.path,
            id,
        )

    def get(self) -> list[GetResponseItem]:
        """
        List TFA configurations of users.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Add a TFA entry for a user.
        """
        return self.client.post(self.path, parameters, parse_as=PostResponseItem)


@dataclass
class AsyncUseridClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, userid: str):
        self.client = client
        self.path = f"{parent_path}/{userid}"

    def id(self, id: str) -> _id.AsyncIdClient:
        return _id.AsyncIdClient(
            self.client,
            self.path,
            id,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        List TFA configurations of users.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Add a TFA entry for a user.
        """
        return await self.client.post(self.path, parameters, parse_as=PostResponseItem)
