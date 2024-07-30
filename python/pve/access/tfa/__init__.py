from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import userid as _userid


class GetResponseItemEntriesItem(BaseModel):
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


class GetResponseItem(BaseModel):
    entries: list[GetResponseItemEntriesItem]
    # Contains a timestamp until when a user is locked out of 2nd factors.
    tfa_locked_until: Optional[int] = Field(alias="tfa-locked-until", default=None)
    # True if the user is currently locked out of TOTP factors.
    totp_locked: Optional[bool] = Field(alias="totp-locked", default=None)
    # User this entry belongs to.
    userid: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class TfaClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'tfa'}"

    def userid(self, userid: str) -> _userid.UseridClient:
        return _userid.UseridClient(
            self.client,
            self.path,
            userid,
        )

    def get(self) -> list[GetResponseItem]:
        """
        List TFA configurations of users.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncTfaClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'tfa'}"

    def userid(self, userid: str) -> _userid.AsyncUseridClient:
        return _userid.AsyncUseridClient(
            self.client,
            self.path,
            userid,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        List TFA configurations of users.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
