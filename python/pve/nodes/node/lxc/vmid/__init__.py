from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import CommonPydanticConfig
from . import clone as _clone
from . import config as _config
from . import feature as _feature
from . import firewall as _firewall
from . import migrate as _migrate
from . import move_volume as _move_volume
from . import mtunnel as _mtunnel
from . import mtunnelwebsocket as _mtunnelwebsocket
from . import pending as _pending
from . import remote_migrate as _remote_migrate
from . import resize as _resize
from . import rrd as _rrd
from . import rrddata as _rrddata
from . import snapshot as _snapshot
from . import spiceproxy as _spiceproxy
from . import status as _status
from . import template as _template
from . import termproxy as _termproxy
from . import vncproxy as _vncproxy
from . import vncwebsocket as _vncwebsocket


class GetResponseItem(BaseModel):
    subdir: str

    class Config(CommonPydanticConfig):
        pass


class DeleteParameters(BaseModel):
    # If set, destroy additionally all disks with the VMID from all enabled storages which are not referenced in the config.
    destroy_unreferenced_disks: Optional[bool] = Field(
        alias="destroy-unreferenced-disks", default=None
    )
    # Force destroy, even if running.
    force: Optional[bool] = Field(default=None)
    # Remove container from all related configurations. For example, backup jobs, replication jobs or HA. Related ACLs and Firewall entries will *always* be removed.
    purge: Optional[bool] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class VmidClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, vmid: str):
        self.client = client
        self.path = f"{parent_path}/{vmid}"

    def config(self) -> _config.ConfigClient:
        return _config.ConfigClient(
            self.client,
            self.path,
        )

    def status(self) -> _status.StatusClient:
        return _status.StatusClient(
            self.client,
            self.path,
        )

    def snapshot(self) -> _snapshot.SnapshotClient:
        return _snapshot.SnapshotClient(
            self.client,
            self.path,
        )

    def firewall(self) -> _firewall.FirewallClient:
        return _firewall.FirewallClient(
            self.client,
            self.path,
        )

    def rrd(self) -> _rrd.RrdClient:
        return _rrd.RrdClient(
            self.client,
            self.path,
        )

    def rrddata(self) -> _rrddata.RrddataClient:
        return _rrddata.RrddataClient(
            self.client,
            self.path,
        )

    def vncproxy(self) -> _vncproxy.VncproxyClient:
        return _vncproxy.VncproxyClient(
            self.client,
            self.path,
        )

    def termproxy(self) -> _termproxy.TermproxyClient:
        return _termproxy.TermproxyClient(
            self.client,
            self.path,
        )

    def vncwebsocket(self) -> _vncwebsocket.VncwebsocketClient:
        return _vncwebsocket.VncwebsocketClient(
            self.client,
            self.path,
        )

    def spiceproxy(self) -> _spiceproxy.SpiceproxyClient:
        return _spiceproxy.SpiceproxyClient(
            self.client,
            self.path,
        )

    def remote_migrate(self) -> _remote_migrate.RemoteMigrateClient:
        return _remote_migrate.RemoteMigrateClient(
            self.client,
            self.path,
        )

    def migrate(self) -> _migrate.MigrateClient:
        return _migrate.MigrateClient(
            self.client,
            self.path,
        )

    def feature(self) -> _feature.FeatureClient:
        return _feature.FeatureClient(
            self.client,
            self.path,
        )

    def template(self) -> _template.TemplateClient:
        return _template.TemplateClient(
            self.client,
            self.path,
        )

    def clone(self) -> _clone.CloneClient:
        return _clone.CloneClient(
            self.client,
            self.path,
        )

    def resize(self) -> _resize.ResizeClient:
        return _resize.ResizeClient(
            self.client,
            self.path,
        )

    def move_volume(self) -> _move_volume.MoveVolumeClient:
        return _move_volume.MoveVolumeClient(
            self.client,
            self.path,
        )

    def pending(self) -> _pending.PendingClient:
        return _pending.PendingClient(
            self.client,
            self.path,
        )

    def mtunnel(self) -> _mtunnel.MtunnelClient:
        return _mtunnel.MtunnelClient(
            self.client,
            self.path,
        )

    def mtunnelwebsocket(self) -> _mtunnelwebsocket.MtunnelwebsocketClient:
        return _mtunnelwebsocket.MtunnelwebsocketClient(
            self.client,
            self.path,
        )

    def delete(self, parameters: DeleteParameters) -> str:
        """
        Destroy the container (also delete all uses files).
        """
        return self.client.delete(self.path, parameters, parse_as=str)

    def get(self) -> list[GetResponseItem]:
        """
        Directory index
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncVmidClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, vmid: str):
        self.client = client
        self.path = f"{parent_path}/{vmid}"

    def config(self) -> _config.AsyncConfigClient:
        return _config.AsyncConfigClient(
            self.client,
            self.path,
        )

    def status(self) -> _status.AsyncStatusClient:
        return _status.AsyncStatusClient(
            self.client,
            self.path,
        )

    def snapshot(self) -> _snapshot.AsyncSnapshotClient:
        return _snapshot.AsyncSnapshotClient(
            self.client,
            self.path,
        )

    def firewall(self) -> _firewall.AsyncFirewallClient:
        return _firewall.AsyncFirewallClient(
            self.client,
            self.path,
        )

    def rrd(self) -> _rrd.AsyncRrdClient:
        return _rrd.AsyncRrdClient(
            self.client,
            self.path,
        )

    def rrddata(self) -> _rrddata.AsyncRrddataClient:
        return _rrddata.AsyncRrddataClient(
            self.client,
            self.path,
        )

    def vncproxy(self) -> _vncproxy.AsyncVncproxyClient:
        return _vncproxy.AsyncVncproxyClient(
            self.client,
            self.path,
        )

    def termproxy(self) -> _termproxy.AsyncTermproxyClient:
        return _termproxy.AsyncTermproxyClient(
            self.client,
            self.path,
        )

    def vncwebsocket(self) -> _vncwebsocket.AsyncVncwebsocketClient:
        return _vncwebsocket.AsyncVncwebsocketClient(
            self.client,
            self.path,
        )

    def spiceproxy(self) -> _spiceproxy.AsyncSpiceproxyClient:
        return _spiceproxy.AsyncSpiceproxyClient(
            self.client,
            self.path,
        )

    def remote_migrate(self) -> _remote_migrate.AsyncRemoteMigrateClient:
        return _remote_migrate.AsyncRemoteMigrateClient(
            self.client,
            self.path,
        )

    def migrate(self) -> _migrate.AsyncMigrateClient:
        return _migrate.AsyncMigrateClient(
            self.client,
            self.path,
        )

    def feature(self) -> _feature.AsyncFeatureClient:
        return _feature.AsyncFeatureClient(
            self.client,
            self.path,
        )

    def template(self) -> _template.AsyncTemplateClient:
        return _template.AsyncTemplateClient(
            self.client,
            self.path,
        )

    def clone(self) -> _clone.AsyncCloneClient:
        return _clone.AsyncCloneClient(
            self.client,
            self.path,
        )

    def resize(self) -> _resize.AsyncResizeClient:
        return _resize.AsyncResizeClient(
            self.client,
            self.path,
        )

    def move_volume(self) -> _move_volume.AsyncMoveVolumeClient:
        return _move_volume.AsyncMoveVolumeClient(
            self.client,
            self.path,
        )

    def pending(self) -> _pending.AsyncPendingClient:
        return _pending.AsyncPendingClient(
            self.client,
            self.path,
        )

    def mtunnel(self) -> _mtunnel.AsyncMtunnelClient:
        return _mtunnel.AsyncMtunnelClient(
            self.client,
            self.path,
        )

    def mtunnelwebsocket(self) -> _mtunnelwebsocket.AsyncMtunnelwebsocketClient:
        return _mtunnelwebsocket.AsyncMtunnelwebsocketClient(
            self.client,
            self.path,
        )

    async def delete(self, parameters: DeleteParameters) -> str:
        """
        Destroy the container (also delete all uses files).
        """
        return await self.client.delete(self.path, parameters, parse_as=str)

    async def get(self) -> list[GetResponseItem]:
        """
        Directory index
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
