from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)
from . import defaults as _defaults
from . import extractconfig as _extractconfig


class PostParameters(BaseModel):
    # Backup all known guest systems on this host.
    all: Optional[bool] = Field(default=None)
    # Limit I/O bandwidth (KBytes per second).
    bwlimit: Optional[int] = Field(default=None)
    # Compress dump file.
    compress: Optional[str] = Field(default=None)
    # Store resulting files to specified directory.
    dumpdir: Optional[str] = Field(default=None)
    # Exclude specified guest systems (assumes --all)
    exclude: Optional[str] = Field(default=None)
    # Exclude certain files/directories (shell globs). Paths starting with '/' are anchored to the container's root,  other paths match relative to each subdirectory.
    exclude_path: Optional[str] = Field(alias="exclude-path", default=None)
    # Set CFQ ionice priority.
    ionice: Optional[int] = Field(default=None)
    # Maximal time to wait for the global lock (minutes).
    lockwait: Optional[int] = Field(default=None)
    # Specify when to send an email
    mailnotification: Optional[str] = Field(default=None)
    # Comma-separated list of email addresses or users that should receive email notifications.
    mailto: Optional[str] = Field(default=None)
    # Deprecated: use 'prune-backups' instead. Maximal number of backup files per guest system.
    maxfiles: Optional[int] = Field(default=None)
    # Backup mode.
    mode: Optional[str] = Field(default=None)
    # Template string for generating notes for the backup(s). It can contain variables which will be replaced by their values. Currently supported are {{cluster}}, {{guestname}}, {{node}}, and {{vmid}}, but more might be added in the future. Needs to be a single line, newline and backslash need to be escaped as '\\n' and '\\\\' respectively.
    notes_template: Optional[str] = Field(alias="notes-template", default=None)
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
    # Write tar to stdout, not to a file.
    stdout: Optional[bool] = Field(default=None)
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
    # Zstd threads. N=0 uses half of the available cores, N>0 uses N as thread count.
    zstd: Optional[int] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


@dataclass
class VzdumpClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'vzdump'}"

    def defaults(self) -> _defaults.DefaultsClient:
        return _defaults.DefaultsClient(
            self.client,
            self.path,
        )

    def extractconfig(self) -> _extractconfig.ExtractconfigClient:
        return _extractconfig.ExtractconfigClient(
            self.client,
            self.path,
        )

    def post(self, parameters: PostParameters) -> str:
        """
        Create backup.
        """
        return self.client.post(self.path, parameters, parse_as=str)


@dataclass
class AsyncVzdumpClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str):
        self.client = client
        self.path = f"{parent_path}/{'vzdump'}"

    def defaults(self) -> _defaults.AsyncDefaultsClient:
        return _defaults.AsyncDefaultsClient(
            self.client,
            self.path,
        )

    def extractconfig(self) -> _extractconfig.AsyncExtractconfigClient:
        return _extractconfig.AsyncExtractconfigClient(
            self.client,
            self.path,
        )

    async def post(self, parameters: PostParameters) -> str:
        """
        Create backup.
        """
        return await self.client.post(self.path, parameters, parse_as=str)
