from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PutParameters(BaseModel):
    # Rule action ('ACCEPT', 'DROP', 'REJECT') or security group name.
    action: Optional[str] = Field(default=None)
    # Descriptive comment.
    comment: Optional[str] = Field(default=None)
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Restrict packet destination address. This can refer to a single IP address, an IP set ('+ipsetname') or an IP alias definition. You can also specify an address range like '20.34.101.207-201.3.9.99', or a list of IP addresses and networks (entries are separated by comma). Please do not mix IPv4 and IPv6 addresses inside such lists.
    dest: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Restrict TCP/UDP destination port. You can use service names or simple numbers (0-65535), as defined in '/etc/services'. Port ranges can be specified with '\\d+:\\d+', for example '80:85', and you can use comma separated list to match several ports or ranges.
    dport: Optional[str] = Field(default=None)
    # Flag to enable/disable a rule.
    enable: Optional[int] = Field(default=None)
    # Specify icmp-type. Only valid if proto equals 'icmp' or 'icmpv6'/'ipv6-icmp'.
    icmp_type: Optional[str] = Field(alias="icmp-type", default=None)
    # Network interface name. You have to use network configuration key names for VMs and containers ('net\\d+'). Host related rules can use arbitrary strings.
    iface: Optional[str] = Field(default=None)
    # Log level for firewall rule.
    log: Optional[str] = Field(default=None)
    # Use predefined standard macro.
    macro: Optional[str] = Field(default=None)
    # Move rule to new position <moveto>. Other arguments are ignored.
    moveto: Optional[int] = Field(default=None)
    # IP protocol. You can use protocol names ('tcp'/'udp') or simple numbers, as defined in '/etc/protocols'.
    proto: Optional[str] = Field(default=None)
    # Restrict packet source address. This can refer to a single IP address, an IP set ('+ipsetname') or an IP alias definition. You can also specify an address range like '20.34.101.207-201.3.9.99', or a list of IP addresses and networks (entries are separated by comma). Please do not mix IPv4 and IPv6 addresses inside such lists.
    source: Optional[str] = Field(default=None)
    # Restrict TCP/UDP source port. You can use service names or simple numbers (0-65535), as defined in '/etc/services'. Port ranges can be specified with '\\d+:\\d+', for example '80:85', and you can use comma separated list to match several ports or ranges.
    sport: Optional[str] = Field(default=None)
    # Rule type.
    type: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    action: str
    comment: Optional[str] = Field(default=None)
    dest: Optional[str] = Field(default=None)
    dport: Optional[str] = Field(default=None)
    enable: Optional[int] = Field(default=None)
    icmp_type: Optional[str] = Field(alias="icmp-type", default=None)
    iface: Optional[str] = Field(default=None)
    ipversion: Optional[int] = Field(default=None)
    # Log level for firewall rule
    log: Optional[str] = Field(default=None)
    macro: Optional[str] = Field(default=None)
    pos: int
    proto: Optional[str] = Field(default=None)
    source: Optional[str] = Field(default=None)
    sport: Optional[str] = Field(default=None)
    type: str

    class Config(CommonPydanticConfig):
        pass


class DeleteParameters(BaseModel):
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class PosClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, pos: str):
        self.client = client
        self.path = f"{parent_path}/{pos}"

    def delete(self, parameters: DeleteParameters):
        """
        Delete rule.
        """
        return self.client.delete(self.path, parameters)

    def get(self) -> GetResponseItem:
        """
        Get single rule data.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Modify rule data.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncPosClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, pos: str):
        self.client = client
        self.path = f"{parent_path}/{pos}"

    async def delete(self, parameters: DeleteParameters):
        """
        Delete rule.
        """
        return await self.client.delete(self.path, parameters)

    async def get(self) -> GetResponseItem:
        """
        Get single rule data.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Modify rule data.
        """
        return await self.client.put(self.path, parameters)
