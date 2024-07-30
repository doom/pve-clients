from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import aplinfo as _aplinfo
from . import apt as _apt
from . import capabilities as _capabilities
from . import ceph as _ceph
from . import certificates as _certificates
from . import config as _config
from . import disks as _disks
from . import dns as _dns
from . import execute as _execute
from . import firewall as _firewall
from . import hardware as _hardware
from . import hosts as _hosts
from . import journal as _journal
from . import lxc as _lxc
from . import migrateall as _migrateall
from . import netstat as _netstat
from . import network as _network
from . import qemu as _qemu
from . import query_url_metadata as _query_url_metadata
from . import replication as _replication
from . import report as _report
from . import rrd as _rrd
from . import rrddata as _rrddata
from . import scan as _scan
from . import sdn as _sdn
from . import services as _services
from . import spiceshell as _spiceshell
from . import startall as _startall
from . import status as _status
from . import stopall as _stopall
from . import storage as _storage
from . import subscription as _subscription
from . import suspendall as _suspendall
from . import syslog as _syslog
from . import tasks as _tasks
from . import termproxy as _termproxy
from . import time as _time
from . import version as _version
from . import vncshell as _vncshell
from . import vncwebsocket as _vncwebsocket
from . import vzdump as _vzdump
from . import wakeonlan as _wakeonlan


class GetResponseItem(BaseModel):
    pass


@dataclass
class NodeClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, node: str):
        self.client = client
        self.path = f"{parent_path}/{node}"

    def qemu(self) -> _qemu.QemuClient:
        return _qemu.QemuClient(
            self.client,
            self.path,
        )

    def lxc(self) -> _lxc.LxcClient:
        return _lxc.LxcClient(
            self.client,
            self.path,
        )

    def ceph(self) -> _ceph.CephClient:
        return _ceph.CephClient(
            self.client,
            self.path,
        )

    def vzdump(self) -> _vzdump.VzdumpClient:
        return _vzdump.VzdumpClient(
            self.client,
            self.path,
        )

    def services(self) -> _services.ServicesClient:
        return _services.ServicesClient(
            self.client,
            self.path,
        )

    def subscription(self) -> _subscription.SubscriptionClient:
        return _subscription.SubscriptionClient(
            self.client,
            self.path,
        )

    def network(self) -> _network.NetworkClient:
        return _network.NetworkClient(
            self.client,
            self.path,
        )

    def tasks(self) -> _tasks.TasksClient:
        return _tasks.TasksClient(
            self.client,
            self.path,
        )

    def scan(self) -> _scan.ScanClient:
        return _scan.ScanClient(
            self.client,
            self.path,
        )

    def hardware(self) -> _hardware.HardwareClient:
        return _hardware.HardwareClient(
            self.client,
            self.path,
        )

    def capabilities(self) -> _capabilities.CapabilitiesClient:
        return _capabilities.CapabilitiesClient(
            self.client,
            self.path,
        )

    def storage(self) -> _storage.StorageClient:
        return _storage.StorageClient(
            self.client,
            self.path,
        )

    def disks(self) -> _disks.DisksClient:
        return _disks.DisksClient(
            self.client,
            self.path,
        )

    def apt(self) -> _apt.AptClient:
        return _apt.AptClient(
            self.client,
            self.path,
        )

    def firewall(self) -> _firewall.FirewallClient:
        return _firewall.FirewallClient(
            self.client,
            self.path,
        )

    def replication(self) -> _replication.ReplicationClient:
        return _replication.ReplicationClient(
            self.client,
            self.path,
        )

    def certificates(self) -> _certificates.CertificatesClient:
        return _certificates.CertificatesClient(
            self.client,
            self.path,
        )

    def config(self) -> _config.ConfigClient:
        return _config.ConfigClient(
            self.client,
            self.path,
        )

    def sdn(self) -> _sdn.SdnClient:
        return _sdn.SdnClient(
            self.client,
            self.path,
        )

    def version(self) -> _version.VersionClient:
        return _version.VersionClient(
            self.client,
            self.path,
        )

    def status(self) -> _status.StatusClient:
        return _status.StatusClient(
            self.client,
            self.path,
        )

    def netstat(self) -> _netstat.NetstatClient:
        return _netstat.NetstatClient(
            self.client,
            self.path,
        )

    def execute(self) -> _execute.ExecuteClient:
        return _execute.ExecuteClient(
            self.client,
            self.path,
        )

    def wakeonlan(self) -> _wakeonlan.WakeonlanClient:
        return _wakeonlan.WakeonlanClient(
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

    def syslog(self) -> _syslog.SyslogClient:
        return _syslog.SyslogClient(
            self.client,
            self.path,
        )

    def journal(self) -> _journal.JournalClient:
        return _journal.JournalClient(
            self.client,
            self.path,
        )

    def vncshell(self) -> _vncshell.VncshellClient:
        return _vncshell.VncshellClient(
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

    def spiceshell(self) -> _spiceshell.SpiceshellClient:
        return _spiceshell.SpiceshellClient(
            self.client,
            self.path,
        )

    def dns(self) -> _dns.DnsClient:
        return _dns.DnsClient(
            self.client,
            self.path,
        )

    def time(self) -> _time.TimeClient:
        return _time.TimeClient(
            self.client,
            self.path,
        )

    def aplinfo(self) -> _aplinfo.AplinfoClient:
        return _aplinfo.AplinfoClient(
            self.client,
            self.path,
        )

    def query_url_metadata(self) -> _query_url_metadata.QueryUrlMetadataClient:
        return _query_url_metadata.QueryUrlMetadataClient(
            self.client,
            self.path,
        )

    def report(self) -> _report.ReportClient:
        return _report.ReportClient(
            self.client,
            self.path,
        )

    def startall(self) -> _startall.StartallClient:
        return _startall.StartallClient(
            self.client,
            self.path,
        )

    def stopall(self) -> _stopall.StopallClient:
        return _stopall.StopallClient(
            self.client,
            self.path,
        )

    def suspendall(self) -> _suspendall.SuspendallClient:
        return _suspendall.SuspendallClient(
            self.client,
            self.path,
        )

    def migrateall(self) -> _migrateall.MigrateallClient:
        return _migrateall.MigrateallClient(
            self.client,
            self.path,
        )

    def hosts(self) -> _hosts.HostsClient:
        return _hosts.HostsClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Node index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncNodeClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, node: str):
        self.client = client
        self.path = f"{parent_path}/{node}"

    def qemu(self) -> _qemu.AsyncQemuClient:
        return _qemu.AsyncQemuClient(
            self.client,
            self.path,
        )

    def lxc(self) -> _lxc.AsyncLxcClient:
        return _lxc.AsyncLxcClient(
            self.client,
            self.path,
        )

    def ceph(self) -> _ceph.AsyncCephClient:
        return _ceph.AsyncCephClient(
            self.client,
            self.path,
        )

    def vzdump(self) -> _vzdump.AsyncVzdumpClient:
        return _vzdump.AsyncVzdumpClient(
            self.client,
            self.path,
        )

    def services(self) -> _services.AsyncServicesClient:
        return _services.AsyncServicesClient(
            self.client,
            self.path,
        )

    def subscription(self) -> _subscription.AsyncSubscriptionClient:
        return _subscription.AsyncSubscriptionClient(
            self.client,
            self.path,
        )

    def network(self) -> _network.AsyncNetworkClient:
        return _network.AsyncNetworkClient(
            self.client,
            self.path,
        )

    def tasks(self) -> _tasks.AsyncTasksClient:
        return _tasks.AsyncTasksClient(
            self.client,
            self.path,
        )

    def scan(self) -> _scan.AsyncScanClient:
        return _scan.AsyncScanClient(
            self.client,
            self.path,
        )

    def hardware(self) -> _hardware.AsyncHardwareClient:
        return _hardware.AsyncHardwareClient(
            self.client,
            self.path,
        )

    def capabilities(self) -> _capabilities.AsyncCapabilitiesClient:
        return _capabilities.AsyncCapabilitiesClient(
            self.client,
            self.path,
        )

    def storage(self) -> _storage.AsyncStorageClient:
        return _storage.AsyncStorageClient(
            self.client,
            self.path,
        )

    def disks(self) -> _disks.AsyncDisksClient:
        return _disks.AsyncDisksClient(
            self.client,
            self.path,
        )

    def apt(self) -> _apt.AsyncAptClient:
        return _apt.AsyncAptClient(
            self.client,
            self.path,
        )

    def firewall(self) -> _firewall.AsyncFirewallClient:
        return _firewall.AsyncFirewallClient(
            self.client,
            self.path,
        )

    def replication(self) -> _replication.AsyncReplicationClient:
        return _replication.AsyncReplicationClient(
            self.client,
            self.path,
        )

    def certificates(self) -> _certificates.AsyncCertificatesClient:
        return _certificates.AsyncCertificatesClient(
            self.client,
            self.path,
        )

    def config(self) -> _config.AsyncConfigClient:
        return _config.AsyncConfigClient(
            self.client,
            self.path,
        )

    def sdn(self) -> _sdn.AsyncSdnClient:
        return _sdn.AsyncSdnClient(
            self.client,
            self.path,
        )

    def version(self) -> _version.AsyncVersionClient:
        return _version.AsyncVersionClient(
            self.client,
            self.path,
        )

    def status(self) -> _status.AsyncStatusClient:
        return _status.AsyncStatusClient(
            self.client,
            self.path,
        )

    def netstat(self) -> _netstat.AsyncNetstatClient:
        return _netstat.AsyncNetstatClient(
            self.client,
            self.path,
        )

    def execute(self) -> _execute.AsyncExecuteClient:
        return _execute.AsyncExecuteClient(
            self.client,
            self.path,
        )

    def wakeonlan(self) -> _wakeonlan.AsyncWakeonlanClient:
        return _wakeonlan.AsyncWakeonlanClient(
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

    def syslog(self) -> _syslog.AsyncSyslogClient:
        return _syslog.AsyncSyslogClient(
            self.client,
            self.path,
        )

    def journal(self) -> _journal.AsyncJournalClient:
        return _journal.AsyncJournalClient(
            self.client,
            self.path,
        )

    def vncshell(self) -> _vncshell.AsyncVncshellClient:
        return _vncshell.AsyncVncshellClient(
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

    def spiceshell(self) -> _spiceshell.AsyncSpiceshellClient:
        return _spiceshell.AsyncSpiceshellClient(
            self.client,
            self.path,
        )

    def dns(self) -> _dns.AsyncDnsClient:
        return _dns.AsyncDnsClient(
            self.client,
            self.path,
        )

    def time(self) -> _time.AsyncTimeClient:
        return _time.AsyncTimeClient(
            self.client,
            self.path,
        )

    def aplinfo(self) -> _aplinfo.AsyncAplinfoClient:
        return _aplinfo.AsyncAplinfoClient(
            self.client,
            self.path,
        )

    def query_url_metadata(self) -> _query_url_metadata.AsyncQueryUrlMetadataClient:
        return _query_url_metadata.AsyncQueryUrlMetadataClient(
            self.client,
            self.path,
        )

    def report(self) -> _report.AsyncReportClient:
        return _report.AsyncReportClient(
            self.client,
            self.path,
        )

    def startall(self) -> _startall.AsyncStartallClient:
        return _startall.AsyncStartallClient(
            self.client,
            self.path,
        )

    def stopall(self) -> _stopall.AsyncStopallClient:
        return _stopall.AsyncStopallClient(
            self.client,
            self.path,
        )

    def suspendall(self) -> _suspendall.AsyncSuspendallClient:
        return _suspendall.AsyncSuspendallClient(
            self.client,
            self.path,
        )

    def migrateall(self) -> _migrateall.AsyncMigrateallClient:
        return _migrateall.AsyncMigrateallClient(
            self.client,
            self.path,
        )

    def hosts(self) -> _hosts.AsyncHostsClient:
        return _hosts.AsyncHostsClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Node index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
