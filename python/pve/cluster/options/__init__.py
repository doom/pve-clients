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
    # Set I/O bandwidth limit for various operations (in KiB/s).
    bwlimit: Optional[str] = Field(default=None)
    # Select the default Console viewer. You can either use the builtin java applet (VNC; deprecated and maps to html5), an external virt-viewer comtatible application (SPICE), an HTML5 based vnc viewer (noVNC), or an HTML5 based console client (xtermjs). If the selected viewer is not available (e.g. SPICE not activated for the VM), the fallback is noVNC.
    console: Optional[str] = Field(default=None)
    # Cluster resource scheduling settings.
    crs: Optional[str] = Field(default=None)
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Datacenter description. Shown in the web-interface datacenter notes panel. This is saved as comment inside the configuration file.
    description: Optional[str] = Field(default=None)
    # Specify email address to send notification from (default is root@$hostname)
    email_from: Optional[str] = Field(default=None)
    # Set the fencing mode of the HA cluster. Hardware mode needs a valid configuration of fence devices in /etc/pve/ha/fence.cfg. With both all two modes are used.  WARNING: 'hardware' and 'both' are EXPERIMENTAL & WIP
    fencing: Optional[str] = Field(default=None)
    # Cluster wide HA settings.
    ha: Optional[str] = Field(default=None)
    # Specify external http proxy which is used for downloads (example: 'http://username:password@host:port/')
    http_proxy: Optional[str] = Field(default=None)
    # Default keybord layout for vnc server.
    keyboard: Optional[str] = Field(default=None)
    # Default GUI language.
    language: Optional[str] = Field(default=None)
    # Prefix for the auto-generated MAC addresses of virtual guests. The default 'BC:24:11' is the OUI assigned by the IEEE to Proxmox Server Solutions GmbH for a 24-bit large MAC block. You're allowed to use this in local networks, i.e., those not directly reachable by the public (e.g., in a LAN or behind NAT).
    mac_prefix: Optional[str] = Field(default=None)
    # Defines how many workers (per node) are maximal started  on actions like 'stopall VMs' or task from the ha-manager.
    max_workers: Optional[int] = Field(default=None)
    # For cluster wide migration settings.
    migration: Optional[str] = Field(default=None)
    # Migration is secure using SSH tunnel by default. For secure private networks you can disable it to speed up migration. Deprecated, use the 'migration' property instead!
    migration_unsecure: Optional[bool] = Field(default=None)
    # Control the range for the free VMID auto-selection pool.
    next_id: Optional[str] = Field(alias="next-id", default=None)
    # Cluster-wide notification settings.
    notify: Optional[str] = Field(default=None)
    # A list of tags that require a `Sys.Modify` on '/' to set and delete. Tags set here that are also in 'user-tag-access' also require `Sys.Modify`.
    registered_tags: Optional[str] = Field(alias="registered-tags", default=None)
    # Tag style options.
    tag_style: Optional[str] = Field(alias="tag-style", default=None)
    # u2f
    u2f: Optional[str] = Field(default=None)
    # Privilege options for user-settable tags
    user_tag_access: Optional[str] = Field(alias="user-tag-access", default=None)
    # webauthn configuration
    webauthn: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
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
        Get datacenter options. Without 'Sys.Audit' on '/' not all options are returned.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters):
        """
        Set datacenter options.
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
        Get datacenter options. Without 'Sys.Audit' on '/' not all options are returned.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters):
        """
        Set datacenter options.
        """
        return await self.client.put(self.path, parameters)
