from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import controller as _controller


class PostParameters(BaseModel):
    # autonomous system number
    asn: Optional[int] = Field(default=None)
    bgp_multipath_as_path_relax: Optional[bool] = Field(
        alias="bgp-multipath-as-path-relax", default=None
    )
    # The SDN controller object identifier.
    controller: str
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
    # Plugin type.
    type: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    controller: str
    pending: Optional[bool] = Field(default=None)
    state: Optional[str] = Field(default=None)
    type: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Display pending config.
    pending: Optional[bool] = Field(default=None)
    # Display running config.
    running: Optional[bool] = Field(default=None)
    # Only list sdn controllers of specific type
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ControllersClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'controllers'}"

    def controller(self, controller: str) -> _controller.ControllerClient:
        return _controller.ControllerClient(
            self.client,
            self.path,
            controller,
        )

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        SDN controllers index.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create a new sdn controller object.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncControllersClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'controllers'}"

    def controller(self, controller: str) -> _controller.AsyncControllerClient:
        return _controller.AsyncControllerClient(
            self.client,
            self.path,
            controller,
        )

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        SDN controllers index.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )

    async def post(self, parameters: PostParameters):
        """
        Create a new sdn controller object.
        """
        return await self.client.post(self.path, parameters)
