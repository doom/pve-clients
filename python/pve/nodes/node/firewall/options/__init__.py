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
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Enable host firewall rules.
    enable: Optional[bool] = Field(default=None)
    # Log level for incoming traffic.
    log_level_in: Optional[str] = Field(default=None)
    # Log level for outgoing traffic.
    log_level_out: Optional[str] = Field(default=None)
    # Enable logging of conntrack information.
    log_nf_conntrack: Optional[bool] = Field(default=None)
    # Enable NDP (Neighbor Discovery Protocol).
    ndp: Optional[bool] = Field(default=None)
    # Allow invalid packets on connection tracking.
    nf_conntrack_allow_invalid: Optional[bool] = Field(default=None)
    # Enable conntrack helpers for specific protocols. Supported protocols: amanda, ftp, irc, netbios-ns, pptp, sane, sip, snmp, tftp
    nf_conntrack_helpers: Optional[str] = Field(default=None)
    # Maximum number of tracked connections.
    nf_conntrack_max: Optional[int] = Field(default=None)
    # Conntrack established timeout.
    nf_conntrack_tcp_timeout_established: Optional[int] = Field(default=None)
    # Conntrack syn recv timeout.
    nf_conntrack_tcp_timeout_syn_recv: Optional[int] = Field(default=None)
    # Enable nftables based firewall (tech preview)
    nftables: Optional[bool] = Field(default=None)
    # Enable SMURFS filter.
    nosmurfs: Optional[bool] = Field(default=None)
    # Enable synflood protection
    protection_synflood: Optional[bool] = Field(default=None)
    # Synflood protection rate burst by ip src.
    protection_synflood_burst: Optional[int] = Field(default=None)
    # Synflood protection rate syn/sec by ip src.
    protection_synflood_rate: Optional[int] = Field(default=None)
    # Log level for SMURFS filter.
    smurf_log_level: Optional[str] = Field(default=None)
    # Log level for illegal tcp flags filter.
    tcp_flags_log_level: Optional[str] = Field(default=None)
    # Filter illegal combinations of TCP flags.
    tcpflags: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Enable host firewall rules.
    enable: Optional[bool] = Field(default=None)
    # Log level for incoming traffic.
    log_level_in: Optional[str] = Field(default=None)
    # Log level for outgoing traffic.
    log_level_out: Optional[str] = Field(default=None)
    # Enable logging of conntrack information.
    log_nf_conntrack: Optional[bool] = Field(default=None)
    # Enable NDP (Neighbor Discovery Protocol).
    ndp: Optional[bool] = Field(default=None)
    # Allow invalid packets on connection tracking.
    nf_conntrack_allow_invalid: Optional[bool] = Field(default=None)
    # Enable conntrack helpers for specific protocols. Supported protocols: amanda, ftp, irc, netbios-ns, pptp, sane, sip, snmp, tftp
    nf_conntrack_helpers: Optional[str] = Field(default=None)
    # Maximum number of tracked connections.
    nf_conntrack_max: Optional[int] = Field(default=None)
    # Conntrack established timeout.
    nf_conntrack_tcp_timeout_established: Optional[int] = Field(default=None)
    # Conntrack syn recv timeout.
    nf_conntrack_tcp_timeout_syn_recv: Optional[int] = Field(default=None)
    # Enable nftables based firewall (tech preview)
    nftables: Optional[bool] = Field(default=None)
    # Enable SMURFS filter.
    nosmurfs: Optional[bool] = Field(default=None)
    # Enable synflood protection
    protection_synflood: Optional[bool] = Field(default=None)
    # Synflood protection rate burst by ip src.
    protection_synflood_burst: Optional[int] = Field(default=None)
    # Synflood protection rate syn/sec by ip src.
    protection_synflood_rate: Optional[int] = Field(default=None)
    # Log level for SMURFS filter.
    smurf_log_level: Optional[str] = Field(default=None)
    # Log level for illegal tcp flags filter.
    tcp_flags_log_level: Optional[str] = Field(default=None)
    # Filter illegal combinations of TCP flags.
    tcpflags: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class OptionsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'options'}"

    def get(self) -> GetResponseItem:
        """
        Get host firewall options.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Set Firewall options.
        """
        return self.client.put(self.path, parameters)


@dataclass
class AsyncOptionsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'options'}"

    async def get(self) -> GetResponseItem:
        """
        Get host firewall options.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Set Firewall options.
        """
        return await self.client.put(self.path, parameters)
