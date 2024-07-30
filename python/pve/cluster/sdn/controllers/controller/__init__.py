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
    # autonomous system number
    asn: Optional[int] = Field(default=None)
    bgp_multipath_as_path_relax: Optional[bool] = Field(
        alias="bgp-multipath-as-path-relax", default=None
    )
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Enable ebgp. (remote-as external)
    ebgp: Optional[bool] = Field(default=None)
    ebgp_multihop: Optional[int] = Field(alias="ebgp-multihop", default=None)
    # ISIS domain.
    isis_domain: Optional[str] = Field(alias="isis-domain", default=None)
    # ISIS interface.
    isis_ifaces: Optional[str] = Field(alias="isis-ifaces", default=None)
    # ISIS network entity title.
    isis_net: Optional[str] = Field(alias="isis-net", default=None)
    # source loopback interface.
    loopback: Optional[str] = Field(default=None)
    # The cluster node name.
    node: Optional[str] = Field(default=None)
    # peers address list.
    peers: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


class GetParameters(BaseModel):
    # Display pending config.
    pending: Optional[bool] = Field(default=None)
    # Display running config.
    running: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ControllerClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, controller: str):
        self.client = client
        self.path = f"{parent_path}/{controller}"

    def delete(self):
        """
        Delete sdn controller object configuration.
        """
        return self.client.delete(
            self.path,
        )

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Read sdn controller configuration.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update sdn controller object configuration.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncControllerClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, controller: str):
        self.client = client
        self.path = f"{parent_path}/{controller}"

    async def delete(self):
        """
        Delete sdn controller object configuration.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Read sdn controller configuration.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update sdn controller object configuration.
        """
        return await self.client.put(self.path, parameters)
