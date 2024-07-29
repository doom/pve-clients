from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import pos as _pos


class PostParameters(BaseModel):
    # Rule action ('ACCEPT', 'DROP', 'REJECT') or security group name.
    action: str
    # Descriptive comment.
    comment: Optional[str] = Field(default=None)
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
    # Update rule at position <pos>.
    pos: Optional[int] = Field(default=None)
    # IP protocol. You can use protocol names ('tcp'/'udp') or simple numbers, as defined in '/etc/protocols'.
    proto: Optional[str] = Field(default=None)
    # Restrict packet source address. This can refer to a single IP address, an IP set ('+ipsetname') or an IP alias definition. You can also specify an address range like '20.34.101.207-201.3.9.99', or a list of IP addresses and networks (entries are separated by comma). Please do not mix IPv4 and IPv6 addresses inside such lists.
    source: Optional[str] = Field(default=None)
    # Restrict TCP/UDP source port. You can use service names or simple numbers (0-65535), as defined in '/etc/services'. Port ranges can be specified with '\\d+:\\d+', for example '80:85', and you can use comma separated list to match several ports or ranges.
    sport: Optional[str] = Field(default=None)
    # Rule type.
    type: str

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pos: int

    class Config(CommonPydanticConfig):
        pass


@dataclass
class GroupClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, group: str):
        self.client = client
        self.path = f"{parent_path}/{group}"

    def pos(self, pos: str) -> _pos.PosClient:
        return _pos.PosClient(
            self.client,
            self.path,
            pos,
        )

    def delete(self):
        """
        Delete security group.
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        List rules.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])

    def post(self, parameters: PostParameters):
        """
        Create new rule.
        """
        return self.client.post(self.path, parameters)


@dataclass
class AsyncGroupClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, group: str):
        self.client = client
        self.path = f"{parent_path}/{group}"

    def pos(self, pos: str) -> _pos.AsyncPosClient:
        return _pos.AsyncPosClient(
            self.client,
            self.path,
            pos,
        )

    async def delete(self):
        """
        Delete security group.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        List rules.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])

    async def post(self, parameters: PostParameters):
        """
        Create new rule.
        """
        return await self.client.post(self.path, parameters)
