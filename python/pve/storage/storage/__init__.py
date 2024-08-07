from dataclasses import dataclass
from typing import Any, Optional

from pydantic import BaseModel, Field, model_serializer, model_validator

from pve.client import AbstractClient, AsyncAbstractClient
from pve.common import (
    CommonPydanticConfig,
    extract_repeated_with_prefix,
    serialize_repeated_with_prefix,
)


class Config(BaseModel):
    """
    Partial, possible server generated, configuration properties.
    """

    # The, possible auto-generated, encryption-key.
    encryption_key: Optional[str] = Field(alias="encryption-key", default=None)

    class Config(CommonPydanticConfig):
        pass


class PutResponseItem(BaseModel):
    # Partial, possible server generated, configuration properties.
    config: Optional[Config] = Field(default=None)
    # The ID of the created storage.
    storage: str
    # The type of the created storage.
    type: str

    class Config(CommonPydanticConfig):
        pass


class PutParameters(BaseModel):
    # block size
    blocksize: Optional[str] = Field(default=None)
    # Set I/O bandwidth limit for various operations (in KiB/s).
    bwlimit: Optional[str] = Field(default=None)
    # host group for comstar views
    comstar_hg: Optional[str] = Field(default=None)
    # target group for comstar views
    comstar_tg: Optional[str] = Field(default=None)
    # Allowed content types.  NOTE: the value 'rootdir' is used for Containers, and value 'images' for VMs.
    content: Optional[str] = Field(default=None)
    # Overrides for default content type directories.
    content_dirs: Optional[str] = Field(alias="content-dirs", default=None)
    # Create the base directory if it doesn't exist.
    create_base_path: Optional[bool] = Field(alias="create-base-path", default=None)
    # Populate the directory with the default structure.
    create_subdirs: Optional[bool] = Field(alias="create-subdirs", default=None)
    # Data Pool (for erasure coding only)
    data_pool: Optional[str] = Field(alias="data-pool", default=None)
    # A list of settings you want to delete.
    delete: Optional[str] = Field(default=None)
    # Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications.
    digest: Optional[str] = Field(default=None)
    # Flag to disable the storage.
    disable: Optional[bool] = Field(default=None)
    # CIFS domain.
    domain: Optional[str] = Field(default=None)
    # Encryption key. Use 'autogen' to generate one automatically without passphrase.
    encryption_key: Optional[str] = Field(alias="encryption-key", default=None)
    # Certificate SHA 256 fingerprint.
    fingerprint: Optional[str] = Field(default=None)
    # Default image format.
    format: Optional[str] = Field(default=None)
    # The Ceph filesystem name.
    fs_name: Optional[str] = Field(alias="fs-name", default=None)
    # Mount CephFS through FUSE.
    fuse: Optional[bool] = Field(default=None)
    # Assume the given path is an externally managed mountpoint and consider the storage offline if it is not mounted. Using a boolean (yes/no) value serves as a shortcut to using the target path in this field.
    is_mountpoint: Optional[str] = Field(default=None)
    # Client keyring contents (for external clusters).
    keyring: Optional[str] = Field(default=None)
    # Always access rbd through krbd kernel module.
    krbd: Optional[bool] = Field(default=None)
    # target portal group for Linux LIO targets
    lio_tpg: Optional[str] = Field(default=None)
    # Base64-encoded, PEM-formatted public RSA key. Used to encrypt a copy of the encryption-key which will be added to each encrypted backup.
    master_pubkey: Optional[str] = Field(alias="master-pubkey", default=None)
    # Maximal number of protected backups per guest. Use '-1' for unlimited.
    max_protected_backups: Optional[int] = Field(
        alias="max-protected-backups", default=None
    )
    # Deprecated: use 'prune-backups' instead. Maximal number of backup files per VM. Use '0' for unlimited.
    maxfiles: Optional[int] = Field(default=None)
    # Create the directory if it doesn't exist and populate it with default sub-dirs. NOTE: Deprecated, use the 'create-base-path' and 'create-subdirs' options instead.
    mkdir: Optional[bool] = Field(default=None)
    # IP addresses of monitors (for external clusters).
    monhost: Optional[str] = Field(default=None)
    # mount point
    mountpoint: Optional[str] = Field(default=None)
    # Namespace.
    namespace: Optional[str] = Field(default=None)
    # Set the NOCOW flag on files. Disables data checksumming and causes data errors to be unrecoverable from while allowing direct I/O. Only use this if data does not need to be any more safe than on a single ext4 formatted disk with no underlying raid system.
    nocow: Optional[bool] = Field(default=None)
    # List of nodes for which the storage configuration applies.
    nodes: Optional[str] = Field(default=None)
    # disable write caching on the target
    nowritecache: Optional[bool] = Field(default=None)
    # NFS/CIFS mount options (see 'man nfs' or 'man mount.cifs')
    options: Optional[str] = Field(default=None)
    # Password for accessing the share/datastore.
    password: Optional[str] = Field(default=None)
    # Pool.
    pool: Optional[str] = Field(default=None)
    # For non default port.
    port: Optional[int] = Field(default=None)
    # Preallocation mode for raw and qcow2 images. Using 'metadata' on raw images results in preallocation=off.
    preallocation: Optional[str] = Field(default=None)
    # The retention options with shorter intervals are processed first with --keep-last being the very first one. Each option covers a specific period of time. We say that backups within this period are covered by this option. The next option does not take care of already covered backups and only considers older backups.
    prune_backups: Optional[str] = Field(alias="prune-backups", default=None)
    # Zero-out data when removing LVs.
    saferemove: Optional[bool] = Field(default=None)
    # Wipe throughput (cstream -t parameter value).
    saferemove_throughput: Optional[str] = Field(default=None)
    # Server IP or DNS name.
    server: Optional[str] = Field(default=None)
    # Backup volfile server IP or DNS name.
    server2: Optional[str] = Field(default=None)
    # Indicate that this is a single storage with the same contents on all nodes (or all listed in the 'nodes' option). It will not make the contents of a local storage automatically accessible to other nodes, it just marks an already shared storage as such!
    shared: Optional[bool] = Field(default=None)
    # Disable TLS certificate verification, only enable on fully trusted networks!
    skip_cert_verification: Optional[bool] = Field(
        alias="skip-cert-verification", default=None
    )
    # SMB protocol version. 'default' if not set, negotiates the highest SMB2+ version supported by both the client and server.
    smbversion: Optional[str] = Field(default=None)
    # use sparse volumes
    sparse: Optional[bool] = Field(default=None)
    # Subdir to mount.
    subdir: Optional[str] = Field(default=None)
    # Only use logical volumes tagged with 'pve-vm-ID'.
    tagged_only: Optional[bool] = Field(default=None)
    # Gluster transport: tcp or rdma
    transport: Optional[str] = Field(default=None)
    # RBD Id.
    username: Optional[str] = Field(default=None)

    class Config(CommonPydanticConfig):
        pass


class GetResponseItem(BaseModel):
    pass


@dataclass
class StorageClient:
    client: AbstractClient
    path: str

    def __init__(self, client: AbstractClient, parent_path: str, storage: str):
        self.client = client
        self.path = f"{parent_path}/{storage}"

    def delete(self):
        """
        Delete storage configuration.
        """
        return self.client.delete(
            self.path,
        )

    def get(self) -> GetResponseItem:
        """
        Read storage configuration.
        """
        return self.client.get(self.path, parse_as=GetResponseItem)

    def put(self, parameters: PutParameters) -> PutResponseItem:
        """
        Update storage configuration.
        """
        return self.client.put(self.path, parameters, parse_as=PutResponseItem)


@dataclass
class AsyncStorageClient:
    client: AsyncAbstractClient
    path: str

    def __init__(self, client: AsyncAbstractClient, parent_path: str, storage: str):
        self.client = client
        self.path = f"{parent_path}/{storage}"

    async def delete(self):
        """
        Delete storage configuration.
        """
        return await self.client.delete(
            self.path,
        )

    async def get(self) -> GetResponseItem:
        """
        Read storage configuration.
        """
        return await self.client.get(self.path, parse_as=GetResponseItem)

    async def put(self, parameters: PutParameters) -> PutResponseItem:
        """
        Update storage configuration.
        """
        return await self.client.put(self.path, parameters, parse_as=PutResponseItem)
