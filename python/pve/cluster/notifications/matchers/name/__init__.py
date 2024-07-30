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
    # Comment
    comment: Optional[str] = Field(default=None)
    # A list of settings you want to delete.
    delete: Optional[list[str]] = Field(default=None)
    # Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
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
    # Targets to notify on match
    target: Optional[list[str]] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Comment
    comment: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
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


@dataclass
class NameClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, name: str):
        self.client = client
        self.path = f"{parent_path}/{name}"

    def delete(self):
        """
        Remove matcher
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> GetResponseItem:
        """
        Return a specific matcher
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update existing matcher
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
        Remove matcher
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> GetResponseItem:
        """
        Return a specific matcher
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update existing matcher
        """
        return await self.client.put(self.path, parameters)
