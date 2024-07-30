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


class PostParameters(BaseModel):
    # Description of the logical PCI device.
    description: Optional[str] = Field(default=None)
    # The ID of the logical PCI mapping.
    id: str
    # A list of maps for the cluster nodes.
    map: list[str]
    # Marks the device(s) as being capable of providing mediated devices.
    mdev: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItemChecksItem(BaseModel):
    # The message of the error
    message: str
    # The severity of the error
    severity: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # A list of checks, only present if 'check_node' is set.
    checks: Optional[list[GetResponseItemChecksItem]] = Field(default=None)
    # A description of the logical mapping.
    description: str
    # The logical ID of the mapping.
    id: str
    # The entries of the mapping.
    map: list[str]

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # If given, checks the configurations on the given node for correctness, and adds relevant diagnostics for the devices to the response.
    check_node: Optional[str] = Field(alias="check-node", default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class PciClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'pci'}"

    def id(self, id: str) -> _id.IdClient:
        return _id.IdClient(
            self.client,
            self.path,
            id,
        )

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List PCI Hardware Mapping
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create a new hardware mapping.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncPciClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'pci'}"

    def id(self, id: str) -> _id.AsyncIdClient:
        return _id.AsyncIdClient(
            self.client,
            self.path,
            id,
        )

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List PCI Hardware Mapping
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )

    async def post(self, parameters: PostParameters):
        """
        Create a new hardware mapping.
        """
        return await self.client.post(self.path, parameters)
