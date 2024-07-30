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
    # Comment
    comment: Optional[str] = Field(default=None)
    # Disable this matcher
    disable: Optional[bool] = Field(default=None)
    # Invert match of the whole matcher
    invert_match: Optional[bool] = Field(alias="invert-match", default=None)
    # Match notification timestamp
    match_calendar: Optional[list[str]] = Field(alias="match-calendar", default=None)
    # Metadata fields to match (regex or exact match). Must be in the form (regex|exact):<field>=<value>
    match_field: Optional[list[str]] = Field(alias="match-field", default=None)
    # Notification severities to match
    match_severity: Optional[list[str]] = Field(alias="match-severity", default=None)
    # Choose between 'all' and 'any' for when multiple properties are specified
    mode: Optional[str] = Field(default=None)
    # Name of the matcher.
    name: str
    # Targets to notify on match
    target: Optional[list[str]] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Comment
    comment: Optional[str] = Field(default=None)
    # Disable this matcher
    disable: Optional[bool] = Field(default=None)
    # Invert match of the whole matcher
    invert_match: Optional[bool] = Field(alias="invert-match", default=None)
    # Match notification timestamp
    match_calendar: Optional[list[str]] = Field(alias="match-calendar", default=None)
    # Metadata fields to match (regex or exact match). Must be in the form (regex|exact):<field>=<value>
    match_field: Optional[list[str]] = Field(alias="match-field", default=None)
    # Notification severities to match
    match_severity: Optional[list[str]] = Field(alias="match-severity", default=None)
    # Choose between 'all' and 'any' for when multiple properties are specified
    mode: Optional[str] = Field(default=None)
    # Name of the matcher.
    name: str
    # Show if this entry was created by a user or was built-in
    origin: str
    # Targets to notify on match
    target: Optional[list[str]] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class MatchersClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'matchers'}"

    def name(self, name: str) -> _name.NameClient:
        return _name.NameClient(
            self.client,
            self.path,
            name,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Returns a list of all matchers
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create a new matcher
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncMatchersClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'matchers'}"

    def name(self, name: str) -> _name.AsyncNameClient:
        return _name.AsyncNameClient(
            self.client,
            self.path,
            name,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Returns a list of all matchers
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters):
        """
        Create a new matcher
        """
        return await self.client.post(self.path, parameters)
