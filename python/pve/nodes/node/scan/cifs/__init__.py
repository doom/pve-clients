from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class GetResponseItem(BaseModel):
    # Descriptive text from server.
    description: str
    # The cifs share name.
    share: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # SMB domain (Workgroup).
    domain: Optional[str] = Field(default=None)
    # User password.
    password: Optional[str] = Field(default=None)
    # The server address (name or IP).
    server: str
    # User name.
    username: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class CifsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'cifs'}"

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Scan remote CIFS server.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])


@dataclass
class AsyncCifsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'cifs'}"

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        Scan remote CIFS server.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )
