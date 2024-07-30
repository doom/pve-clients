from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetResponseWarningsItem(BaseModel):
    # Related subject (config) key of warning.
    key: Optional[str] = Field(default=None)
    # What this warning is about.
    type: str
    # Related subject (config) value of warning.
    value: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class Net(BaseModel):
    """
    Recognised network interfaces as `net$id` => { ...params } object.
    """

    pass


class Disks(BaseModel):
    """
    Recognised disk volumes as `$bus$id` => `$storeid:$path` map.
    """

    pass


class CreateArgs(BaseModel):
    """
    Parameters which can be used in a call to create a VM or container.
    """

    pass


class GetResponseItem(BaseModel):
    """
    Information about how to import a guest.
    """

    # Parameters which can be used in a call to create a VM or container.
    create_args: CreateArgs = Field(alias="create-args")
    # Recognised disk volumes as `$bus$id` => `$storeid:$path` map.
    disks: Optional[Disks] = Field(default=None)
    # Recognised network interfaces as `net$id` => { ...params } object.
    net: Optional[Net] = Field(default=None)
    # The type of the import-source of this guest volume.
    source: str
    # The type of guest this is going to produce.
    type: str
    # List of known issues that can affect the import of a guest. Note that lack of warning does not imply that there cannot be any problems.
    warnings: Optional[list[GetResponseWarningsItem]] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Volume identifier for the guest archive/entry.
    volume: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ImportMetadataClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'import-metadata'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get the base parameters for creating a guest which imports data from a foreign importable guest, like an ESXi VM
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncImportMetadataClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'import-metadata'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get the base parameters for creating a guest which imports data from a foreign importable guest, like an ESXi VM
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
