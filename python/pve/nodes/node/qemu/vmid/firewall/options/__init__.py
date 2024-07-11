from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig


class PutParameters(BaseModel):
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Enable DHCP.
    dhcp: Optional[bool] = Field(default=None)
    # Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Enable/disable firewall rules.
    enable: Optional[bool] = Field(default=None)
    # Enable default IP filters. This is equivalent to adding an empty ipfilter-net<id> ipset for every interface. Such ipsets implicitly contain sane default restrictions such as restricting IPv6 link local addresses to the one derived from the interface's MAC address. For containers the configured IP addresses will be implicitly added.
    ipfilter: Optional[bool] = Field(default=None)
    # Log level for incoming traffic.
    log_level_in: Optional[str] = Field(default=None)
    # Log level for outgoing traffic.
    log_level_out: Optional[str] = Field(default=None)
    # Enable/disable MAC address filter.
    macfilter: Optional[bool] = Field(default=None)
    # Enable NDP (Neighbor Discovery Protocol).
    ndp: Optional[bool] = Field(default=None)
    # Input policy.
    policy_in: Optional[str] = Field(default=None)
    # Output policy.
    policy_out: Optional[str] = Field(default=None)
    # Allow sending Router Advertisement.
    radv: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    # Enable DHCP.
    dhcp: Optional[bool] = Field(default=None)
    # Enable/disable firewall rules.
    enable: Optional[bool] = Field(default=None)
    # Enable default IP filters. This is equivalent to adding an empty ipfilter-net<id> ipset for every interface. Such ipsets implicitly contain sane default restrictions such as restricting IPv6 link local addresses to the one derived from the interface's MAC address. For containers the configured IP addresses will be implicitly added.
    ipfilter: Optional[bool] = Field(default=None)
    # Log level for incoming traffic.
    log_level_in: Optional[str] = Field(default=None)
    # Log level for outgoing traffic.
    log_level_out: Optional[str] = Field(default=None)
    # Enable/disable MAC address filter.
    macfilter: Optional[bool] = Field(default=None)
    # Enable NDP (Neighbor Discovery Protocol).
    ndp: Optional[bool] = Field(default=None)
    # Input policy.
    policy_in: Optional[str] = Field(default=None)
    # Output policy.
    policy_out: Optional[str] = Field(default=None)
    # Allow sending Router Advertisement.
    radv: Optional[bool] = Field(default=None)

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
        Get VM firewall options.
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
        Get VM firewall options.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Set Firewall options.
        """
        return await self.client.put(self.path, parameters)
