from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import iface as _iface


class PostParameters(BaseModel):
    # IP address.
    address: Optional[str] = Field(default=None)
    # IP address.
    address6: Optional[str] = Field(default=None)
    # Automatically start interface on boot.
    autostart: Optional[bool] = Field(default=None)
    # Specify the primary interface for active-backup bond.
    bond_primary: Optional[str] = Field(alias="bond-primary", default=None)
    # Bonding mode.
    bond_mode: Optional[str] = Field(default=None)
    # Selects the transmit hash policy to use for slave selection in balance-xor and 802.3ad modes.
    bond_xmit_hash_policy: Optional[str] = Field(default=None)
    # Specify the interfaces you want to add to your bridge.
    bridge_ports: Optional[str] = Field(default=None)
    # Enable bridge vlan support.
    bridge_vlan_aware: Optional[bool] = Field(default=None)
    # IPv4 CIDR.
    cidr: Optional[str] = Field(default=None)
    # IPv6 CIDR.
    cidr6: Optional[str] = Field(default=None)
    # Comments
    comments: Optional[str] = Field(default=None)
    # Comments
    comments6: Optional[str] = Field(default=None)
    # Default gateway address.
    gateway: Optional[str] = Field(default=None)
    # Default ipv6 gateway address.
    gateway6: Optional[str] = Field(default=None)
    # Network interface name.
    iface: str
    # MTU.
    mtu: Optional[int] = Field(default=None)
    # Network mask.
    netmask: Optional[str] = Field(default=None)
    # Network mask.
    netmask6: Optional[int] = Field(default=None)
    # Specify the interfaces used by the bonding device.
    ovs_bonds: Optional[str] = Field(default=None)
    # The OVS bridge associated with a OVS port. This is required when you create an OVS port.
    ovs_bridge: Optional[str] = Field(default=None)
    # OVS interface options.
    ovs_options: Optional[str] = Field(default=None)
    # Specify the interfaces you want to add to your bridge.
    ovs_ports: Optional[str] = Field(default=None)
    # Specify a VLan tag (used by OVSPort, OVSIntPort, OVSBond)
    ovs_tag: Optional[int] = Field(default=None)
    # Specify the interfaces used by the bonding device.
    slaves: Optional[str] = Field(default=None)
    # Network interface type
    type: str
    # vlan-id for a custom named vlan interface (ifupdown2 only).
    vlan_id: Optional[int] = Field(alias="vlan-id", default=None)
    # Specify the raw interface for the vlan interface.
    vlan_raw_device: Optional[str] = Field(alias="vlan-raw-device", default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


class GetParameters(BaseModel):
    # Only list specific interface types.
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class NetworkClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'network'}"

    def iface(self, iface: str) -> _iface.IfaceClient:
        return _iface.IfaceClient(
            self.client,
            self.path,
            iface,
        )

    def delete(self):
        """
        Revert network configuration changes.
        """
        return self.client.delete(
            self.path,
        )

    def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List available networks
        """
        return self.client.get(self.path, parameters, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create network device configuration
        """
        return self.client.post(self.path, parameters)

    def put(self) -> str:
        """
        Reload network configuration
        """
        return self.client.put(self.path, parse_as=str)


@dataclass
class AsyncNetworkClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'network'}"

    def iface(self, iface: str) -> _iface.AsyncIfaceClient:
        return _iface.AsyncIfaceClient(
            self.client,
            self.path,
            iface,
        )

    async def delete(self):
        """
        Revert network configuration changes.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self, parameters: GetParameters) -> list[GetResponseItem]:
        """
        List available networks
        """
        return await self.client.get(
            self.path, parameters, parse_as=list[GetResponseItem]
        )

    async def post(self, parameters: PostParameters):
        """
        Create network device configuration
        """
        return await self.client.post(self.path, parameters)

    async def put(self) -> str:
        """
        Reload network configuration
        """
        return await self.client.put(self.path, parse_as=str)
