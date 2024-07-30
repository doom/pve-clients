from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class GetResponseItem(BaseModel):
    # Backup all known guest systems on this host.
    all: Optional[bool] = Field(default=None)
    # Limit I/O bandwidth (in KiB/s).
    bwlimit: Optional[int] = Field(default=None)
    # Compress dump file.
    compress: Optional[str] = Field(default=None)
    # Store resulting files to specified directory.
    dumpdir: Optional[str] = Field(default=None)
    # Exclude specified guest systems (assumes --all)
    exclude: Optional[str] = Field(default=None)
    # Exclude certain files/directories (shell globs). Paths starting with '/' are anchored to the container's root, other paths match relative to each subdirectory.
    exclude_path: Optional[list[str]] = Field(alias="exclude-path", default=None)
    # Options for backup fleecing (VM only).
    fleecing: Optional[str] = Field(default=None)
    # Set IO priority when using the BFQ scheduler. For snapshot and suspend mode backups of VMs, this only affects the compressor. A value of 8 means the idle priority is used, otherwise the best-effort priority is used with the specified value.
    ionice: Optional[int] = Field(default=None)
    # Maximal time to wait for the global lock (minutes).
    lockwait: Optional[int] = Field(default=None)
    # Deprecated: use notification targets/matchers instead. Specify when to send a notification mail
    mailnotification: Optional[str] = Field(default=None)
    # Deprecated: Use notification targets/matchers instead. Comma-separated list of email addresses or users that should receive email notifications.
    mailto: Optional[str] = Field(default=None)
    # Deprecated: use 'prune-backups' instead. Maximal number of backup files per guest system.
    maxfiles: Optional[int] = Field(default=None)
    # Backup mode.
    mode: Optional[str] = Field(default=None)
    # Only run if executed on this node.
    node: Optional[str] = Field(default=None)
    # Template string for generating notes for the backup(s). It can contain variables which will be replaced by their values. Currently supported are {{cluster}}, {{guestname}}, {{node}}, and {{vmid}}, but more might be added in the future. Needs to be a single line, newline and backslash need to be escaped as '\\n' and '\\\\' respectively.
    notes_template: Optional[str] = Field(alias="notes-template", default=None)
    # Determine which notification system to use. If set to 'legacy-sendmail', vzdump will consider the mailto/mailnotification parameters and send emails to the specified address(es) via the 'sendmail' command. If set to 'notification-system', a notification will be sent via PVE's notification system, and the mailto and mailnotification will be ignored. If set to 'auto' (default setting), an email will be sent if mailto is set, and the notification system will be used if not.
    notification_mode: Optional[str] = Field(alias="notification-mode", default=None)
    # Deprecated: Do not use
    notification_policy: Optional[str] = Field(
        alias="notification-policy", default=None
    )
    # Deprecated: Do not use
    notification_target: Optional[str] = Field(
        alias="notification-target", default=None
    )
    # EXPERIMENTAL: PBS mode used to detect file changes and switch encoding format for container backups.
    pbs_change_detection_mode: Optional[str] = Field(
        alias="pbs-change-detection-mode", default=None
    )
    # Other performance-related settings.
    performance: Optional[str] = Field(default=None)
    # Use pigz instead of gzip when N>0. N=1 uses half of cores, N>1 uses N as thread count.
    pigz: Optional[int] = Field(default=None)
    # Backup all known guest systems included in the specified pool.
    pool: Optional[str] = Field(default=None)
    # If true, mark backup(s) as protected.
    protected: Optional[bool] = Field(default=None)
    # Use these retention options instead of those from the storage configuration.
    prune_backups: Optional[str] = Field(alias="prune-backups", default=None)
    # Be quiet.
    quiet: Optional[bool] = Field(default=None)
    # Prune older backups according to 'prune-backups'.
    remove: Optional[bool] = Field(default=None)
    # Use specified hook script.
    script: Optional[str] = Field(default=None)
    # Exclude temporary files and logs.
    stdexcludes: Optional[bool] = Field(default=None)
    # Stop running backup jobs on this host.
    stop: Optional[bool] = Field(default=None)
    # Maximal time to wait until a guest system is stopped (minutes).
    stopwait: Optional[int] = Field(default=None)
    # Store resulting file to this storage.
    storage: Optional[str] = Field(default=None)
    # Store temporary files to specified directory.
    tmpdir: Optional[str] = Field(default=None)
    # The ID of the guest system you want to backup.
    vmid: Optional[str] = Field(default=None)
    # Zstd threads. N=0 uses half of the available cores, if N is set to a value bigger than 0, N is used as thread count.
    zstd: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetParameters(BaseModel):
    # The storage identifier.
    storage: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class DefaultsClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'defaults'}"

    def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get the currently configured vzdump defaults.
        """
        return self.client.get(self.path, parameters, parse_as=GetResponseItem)


@dataclass
class AsyncDefaultsClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'defaults'}"

    async def get(self, parameters: GetParameters) -> GetResponseItem:
        """
        Get the currently configured vzdump defaults.
        """
        return await self.client.get(self.path, parameters, parse_as=GetResponseItem)
