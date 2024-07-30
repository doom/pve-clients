from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import zone as _zone


class PostParameters(BaseModel):
    # Advertise evpn subnets if you have silent hosts
    advertise_subnets: Optional[bool] = Field(alias="advertise-subnets", default=None)
    bridge: Optional[str] = Field(default=None)
    # Disable auto mac learning.
    bridge_disable_mac_learning: Optional[bool] = Field(
        alias="bridge-disable-mac-learning", default=None
    )
    # Frr router name
    controller: Optional[str] = Field(default=None)
    # Type of the DHCP backend for this zone
    dhcp: Optional[str] = Field(default=None)
    # Disable ipv4 arp && ipv6 neighbour discovery suppression
    disable_arp_nd_suppression: Optional[bool] = Field(
        alias="disable-arp-nd-suppression", default=None
    )
    # dns api server
    dns: Optional[str] = Field(default=None)
    # dns domain zone  ex: mydomain.com
    dnszone: Optional[str] = Field(default=None)
    # Faucet dataplane id
    dp_id: Optional[int] = Field(alias="dp-id", default=None)
    # List of cluster node names.
    exitnodes: Optional[str] = Field(default=None)
    # Allow exitnodes to connect to evpn guests
    exitnodes_local_routing: Optional[bool] = Field(
        alias="exitnodes-local-routing", default=None
    )
    # Force traffic to this exitnode first.
    exitnodes_primary: Optional[str] = Field(alias="exitnodes-primary", default=None)
    # use a specific ipam
    ipam: Optional[str] = Field(default=None)
    # Anycast logical router mac address
    mac: Optional[str] = Field(default=None)
    # MTU
    mtu: Optional[int] = Field(default=None)
    # List of cluster node names.
    nodes: Optional[str] = Field(default=None)
    # peers address list.
    peers: Optional[str] = Field(default=None)
    # reverse dns api server
    reversedns: Optional[str] = Field(default=None)
    # Route-Target import
    rt_import: Optional[str] = Field(alias="rt-import", default=None)
    # Service-VLAN Tag
    tag: Optional[int] = Field(default=None)
    # Plugin type.
    type: str
    vlan_protocol: Optional[str] = Field(alias="vlan-protocol", default=None)
    # l3vni.
    vrf_vxlan: Optional[int] = Field(alias="vrf-vxlan", default=None)
    # Vxlan tunnel udp port (default 4789).
    vxlan_port: Optional[int] = Field(alias="vxlan-port", default=None)
    # The SDN zone object identifier.
    zone: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    dhcp: Optional[str] = Field(default=None)
    dns: Optional[str] = Field(default=None)
    dnszone: Optional[str] = Field(default=None)
    ipam: Optional[str] = Field(default=None)
    mtu: Optional[int] = Field(default=None)
    nodes: Optional[str] = Field(default=None)
    pending: Optional[bool] = Field(default=None)
    reversedns: Optional[str] = Field(default=None)
    state: Optional[str] = Field(default=None)
    type: str
    zone: str

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # Display pending config.
    pending: Optional[bool] = Field(default=None)
    # Display running config.
    running: Optional[bool] = Field(default=None)
    # Only list SDN zones of specific type
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class ZonesClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'zones'}"

    def zone(self, zone: str) -> _zone.ZoneClient:
        return _zone.ZoneClient(
            self.client,
            self.path,
            zone,
        )

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        SDN zones index.
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create a new sdn zone object.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncZonesClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'zones'}"

    def zone(self, zone: str) -> _zone.AsyncZoneClient:
        return _zone.AsyncZoneClient(
            self.client,
            self.path,
            zone,
        )

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        SDN zones index.
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )

    async def post(self, parameters: PostParameters):
        """
        Create a new sdn zone object.
        """
        return await self.client.post(self.path, parameters)
