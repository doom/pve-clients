from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PostResponseItem(BaseModel):
    """
    Returns an object with a single `result` property.
    """

    pass


class PostParameters(BaseModel):
    # set to 1 if the password has already been passed through crypt()
    crypted: Optional[bool] = Field(default=None)
    # The new password.
    password: str
    # The user to set the password for.
    username: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class SetUserPasswordClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'set-user-password'}"

    def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Sets the password for the given user to the given password
        """
        return self.client.post(self.path, parameters, parse_as=PostResponseItem)


@dataclass
class AsyncSetUserPasswordClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'set-user-password'}"

    async def post(self, parameters: PostParameters) -> PostResponseItem:
        """
        Sets the password for the given user to the given password
        """
        return await self.client.post(self.path, parameters, parse_as=PostResponseItem)
