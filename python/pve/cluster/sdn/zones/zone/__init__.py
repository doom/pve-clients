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
    # Advertise evpn subnets if you have silent hosts
    advertise_subnets: Optional[bool] = Field(alias="advertise-subnets", default=None)
    bridge: Optional[str] = Field(default=None)
    # Disable auto mac learning.
    bridge_disable_mac_learning: Optional[bool] = Field(
        alias="bridge-disable-mac-learning", default=None
    )
    # Frr router name
    controller: Optional[str] = Field(default=None)
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Type of the DHCP backend for this zone
    dhcp: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
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
    vlan_protocol: Optional[str] = Field(alias="vlan-protocol", default=None)
    # l3vni.
    vrf_vxlan: Optional[int] = Field(alias="vrf-vxlan", default=None)
    # Vxlan tunnel udp port (default 4789).
    vxlan_port: Optional[int] = Field(alias="vxlan-port", default=None)

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
class ZoneClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, zone: str):
        self.client = client
        self.path = f"{parent_path}/{zone}"

    def delete(self):
        """
        Delete sdn zone object configuration.
        """
        return self.client.delete(
            self.path,
        )

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Read sdn zone configuration.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update sdn zone object configuration.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncZoneClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, zone: str):
        self.client = client
        self.path = f"{parent_path}/{zone}"

    async def delete(self):
        """
        Delete sdn zone object configuration.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Read sdn zone configuration.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update sdn zone object configuration.
        """
        return await self.client.put(self.path, parameters)
