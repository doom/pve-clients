from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import userid as _userid


class PostResponseItem(BaseModel):
    ticket: str

    class Config(CommonPydanticConfig):
        pass


class PostParameters(BaseModel):
    # The response to the current authentication challenge.
    response: str

    class Config(CommonPydanticConfig):
        pass


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

    def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Finish a u2f challenge.
        """
        return self.client.post(self.path, parameters, parse_as=PostResponseItem)


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

    async def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Finish a u2f challenge.
        """
        return await self.client.post(self.path, parameters, parse_as=PostResponseItem)
