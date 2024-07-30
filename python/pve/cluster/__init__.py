from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import acme as _acme
from . import backup as _backup
from . import backup_info as _backup_info
from . import ceph as _ceph
from . import config as _config
from . import firewall as _firewall
from . import ha as _ha
from . import jobs as _jobs
from . import log as _log
from . import mapping as _mapping
from . import metrics as _metrics
from . import nextid as _nextid
from . import notifications as _notifications
from . import options as _options
from . import replication as _replication
from . import resources as _resources
from . import sdn as _sdn
from . import status as _status
from . import tasks as _tasks


class GetResponseItem(BaseModel):
    pass


@dataclass
class ClusterClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient):
        self.client = client
        self.path = "cluster"

    def replication(self) -> _replication.ReplicationClient:
        return _replication.ReplicationClient(
            self.client,
            self.path,
        )

    def metrics(self) -> _metrics.MetricsClient:
        return _metrics.MetricsClient(
            self.client,
            self.path,
        )

    def notifications(self) -> _notifications.NotificationsClient:
        return _notifications.NotificationsClient(
            self.client,
            self.path,
        )

    def config(self) -> _config.ConfigClient:
        return _config.ConfigClient(
            self.client,
            self.path,
        )

    def firewall(self) -> _firewall.FirewallClient:
        return _firewall.FirewallClient(
            self.client,
            self.path,
        )

    def backup(self) -> _backup.BackupClient:
        return _backup.BackupClient(
            self.client,
            self.path,
        )

    def backup_info(self) -> _backup_info.BackupInfoClient:
        return _backup_info.BackupInfoClient(
            self.client,
            self.path,
        )

    def ha(self) -> _ha.HaClient:
        return _ha.HaClient(
            self.client,
            self.path,
        )

    def acme(self) -> _acme.AcmeClient:
        return _acme.AcmeClient(
            self.client,
            self.path,
        )

    def ceph(self) -> _ceph.CephClient:
        return _ceph.CephClient(
            self.client,
            self.path,
        )

    def jobs(self) -> _jobs.JobsClient:
        return _jobs.JobsClient(
            self.client,
            self.path,
        )

    def mapping(self) -> _mapping.MappingClient:
        return _mapping.MappingClient(
            self.client,
            self.path,
        )

    def sdn(self) -> _sdn.SdnClient:
        return _sdn.SdnClient(
            self.client,
            self.path,
        )

    def log(self) -> _log.LogClient:
        return _log.LogClient(
            self.client,
            self.path,
        )

    def resources(self) -> _resources.ResourcesClient:
        return _resources.ResourcesClient(
            self.client,
            self.path,
        )

    def tasks(self) -> _tasks.TasksClient:
        return _tasks.TasksClient(
            self.client,
            self.path,
        )

    def options(self) -> _options.OptionsClient:
        return _options.OptionsClient(
            self.client,
            self.path,
        )

    def status(self) -> _status.StatusClient:
        return _status.StatusClient(
            self.client,
            self.path,
        )

    def nextid(self) -> _nextid.NextidClient:
        return _nextid.NextidClient(
            self.client,
            self.path,
        )

    def get(self) -> list[GetResponseItem]:
        """
        Cluster index.
        """
        return self.client.get(self.path, parse_as=list[GetResponseItem])


@dataclass
class AsyncClusterClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient):
        self.client = client
        self.path = "cluster"

    def replication(self) -> _replication.AsyncReplicationClient:
        return _replication.AsyncReplicationClient(
            self.client,
            self.path,
        )

    def metrics(self) -> _metrics.AsyncMetricsClient:
        return _metrics.AsyncMetricsClient(
            self.client,
            self.path,
        )

    def notifications(self) -> _notifications.AsyncNotificationsClient:
        return _notifications.AsyncNotificationsClient(
            self.client,
            self.path,
        )

    def config(self) -> _config.AsyncConfigClient:
        return _config.AsyncConfigClient(
            self.client,
            self.path,
        )

    def firewall(self) -> _firewall.AsyncFirewallClient:
        return _firewall.AsyncFirewallClient(
            self.client,
            self.path,
        )

    def backup(self) -> _backup.AsyncBackupClient:
        return _backup.AsyncBackupClient(
            self.client,
            self.path,
        )

    def backup_info(self) -> _backup_info.AsyncBackupInfoClient:
        return _backup_info.AsyncBackupInfoClient(
            self.client,
            self.path,
        )

    def ha(self) -> _ha.AsyncHaClient:
        return _ha.AsyncHaClient(
            self.client,
            self.path,
        )

    def acme(self) -> _acme.AsyncAcmeClient:
        return _acme.AsyncAcmeClient(
            self.client,
            self.path,
        )

    def ceph(self) -> _ceph.AsyncCephClient:
        return _ceph.AsyncCephClient(
            self.client,
            self.path,
        )

    def jobs(self) -> _jobs.AsyncJobsClient:
        return _jobs.AsyncJobsClient(
            self.client,
            self.path,
        )

    def mapping(self) -> _mapping.AsyncMappingClient:
        return _mapping.AsyncMappingClient(
            self.client,
            self.path,
        )

    def sdn(self) -> _sdn.AsyncSdnClient:
        return _sdn.AsyncSdnClient(
            self.client,
            self.path,
        )

    def log(self) -> _log.AsyncLogClient:
        return _log.AsyncLogClient(
            self.client,
            self.path,
        )

    def resources(self) -> _resources.AsyncResourcesClient:
        return _resources.AsyncResourcesClient(
            self.client,
            self.path,
        )

    def tasks(self) -> _tasks.AsyncTasksClient:
        return _tasks.AsyncTasksClient(
            self.client,
            self.path,
        )

    def options(self) -> _options.AsyncOptionsClient:
        return _options.AsyncOptionsClient(
            self.client,
            self.path,
        )

    def status(self) -> _status.AsyncStatusClient:
        return _status.AsyncStatusClient(
            self.client,
            self.path,
        )

    def nextid(self) -> _nextid.AsyncNextidClient:
        return _nextid.AsyncNextidClient(
            self.client,
            self.path,
        )

    async def get(self) -> list[GetResponseItem]:
        """
        Cluster index.
        """
        return await self.client.get(self.path, parse_as=list[GetResponseItem])
