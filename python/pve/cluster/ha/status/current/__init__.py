from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetResponseItem(BaseModel):
    # For type 'service'. Service state as seen by the CRM.
    crm_state: Optional[str] = Field(default=None)
    # Status entry ID (quorum, master, lrm:<node>, service:<sid>).
    id: str
    # For type 'service'.
    max_relocate: Optional[int] = Field(default=None)
    # For type 'service'.
    max_restart: Optional[int] = Field(default=None)
    # Node associated to status entry.
    node: str
    # For type 'quorum'. Whether the cluster is quorate or not.
    quorate: Optional[bool] = Field(default=None)
    # For type 'service'. Requested service state.
    request_state: Optional[str] = Field(default=None)
    # For type 'service'. Service ID.
    sid: Optional[str] = Field(default=None)
    # For type 'service'. Verbose service state.
    state: Optional[str] = Field(default=None)
    # Status of the entry (value depends on type).
    status: str
    # For type 'lrm','master'. Timestamp of the status information.
    timestamp: Optional[int] = Field(default=None)
    # Type of status entry.
    type: dict[str, Any]

    class Config(CommonPydanticConfig):
        pass


@dataclass
class CurrentClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'current'}"

    def get(self) -> list[GetResponseItem]:
        """
        Get HA manger status.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncCurrentClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'current'}"

    async def get(self) -> list[GetResponseItem]:
        """
        Get HA manger status.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
