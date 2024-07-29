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
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Default gateway address.
    gateway: Optional[str] = Field(default=None)
    # Default ipv6 gateway address.
    gateway6: Optional[str] = Field(default=None)
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
    method: str
    type: str

    class Config(CommonPydanticConfig):
        pass


@dataclass
class IfaceClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, iface: str):
        self.client = client
        self.path = f"{parent_path}/{iface}"

    def delete(self):
        """
        Delete network device configuration
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> GetResponseItem:
        """
        Read network device configuration
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Update network device configuration
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncIfaceClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, iface: str):
        self.client = client
        self.path = f"{parent_path}/{iface}"

    async def delete(self):
        """
        Delete network device configuration
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> GetResponseItem:
        """
        Read network device configuration
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Update network device configuration
        """
        return await self.client.put(self.path, parameters)
