from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetParameters(BaseModel):
    # End before the given Cursor. Conflicts with 'until'
    endcursor: Optional[str] = Field(default=None)
    # Limit to the last X lines. Conflicts with a range.
    lastentries: Optional[int] = Field(default=None)
    # Display all log since this UNIX epoch. Conflicts with 'startcursor'.
    since: Optional[int] = Field(default=None)
    # Start after the given Cursor. Conflicts with 'since'
    startcursor: Optional[str] = Field(default=None)
    # Display all log until this UNIX epoch. Conflicts with 'endcursor'.
    until: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class JournalClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'journal'}"

    def get(self, parameters: GetParameters) -> list[str]:
        """
        Read Journal
        """
        return self.client.get(self.path, parameters, parse_as=list[str])


@dataclass
class AsyncJournalClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'journal'}"

    async def get(self, parameters: GetParameters) -> list[str]:
        """
        Read Journal
        """
        return await self.client.get(self.path, parameters, parse_as=list[str])
