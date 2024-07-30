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
    # A description to distinguish multiple entries from one another
    description: Optional[str] = Field(default=None)
    # Whether the entry should be enabled for login.
    enable: Optional[bool] = Field(default=None)
    # The current password of the user performing the change.
    password: Optional[str] = Field(default=None)

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


class DeleteParameters(BaseModel):
    # The current password of the user performing the change.
    password: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class IdClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, id: str):
        self.client = client
        self.path = f"{parent_path}/{id}"

    def delete(self, parameters: DeleteParameters):
        """
        Delete a TFA entry by ID.
        """
        return self.client.delete(self.path, parameters)

    def get(self) -> GetResponseItem:
        """
        Fetch a requested TFA entry if present.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Add a TFA entry for a user.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncIdClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, id: str):
        self.client = client
        self.path = f"{parent_path}/{id}"

    async def delete(self, parameters: DeleteParameters):
        """
        Delete a TFA entry by ID.
        """
        return await self.client.delete(self.path, parameters)

    async def get(self) -> GetResponseItem:
        """
        Fetch a requested TFA entry if present.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Add a TFA entry for a user.
        """
        return await self.client.put(self.path, parameters)
