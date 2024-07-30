#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "block size"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub blocksize: Option<String>,
    #[doc = "Set I/O bandwidth limit for various operations (in KiB/s)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bwlimit: Option<String>,
    #[doc = "host group for comstar views"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comstar_hg: Option<String>,
    #[doc = "target group for comstar views"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comstar_tg: Option<String>,
    #[doc = "Allowed content types.  NOTE: the value 'rootdir' is used for Containers, and value 'images' for VMs. "]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub content: Option<String>,
    #[doc = "Overrides for default content type directories."]
    #[serde(
        rename = "content-dirs",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub content_dirs: Option<String>,
    #[doc = "Create the base directory if it doesn't exist."]
    #[serde(
        rename = "create-base-path",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub create_base_path: Option<bool>,
    #[doc = "Populate the directory with the default structure."]
    #[serde(
        rename = "create-subdirs",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub create_subdirs: Option<bool>,
    #[doc = "Data Pool (for erasure coding only)"]
    #[serde(rename = "data-pool", skip_serializing_if = "Option::is_none", default)]
    pub data_pool: Option<String>,
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Flag to disable the storage."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub disable: Option<bool>,
    #[doc = "CIFS domain."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub domain: Option<String>,
    #[doc = "Encryption key. Use 'autogen' to generate one automatically without passphrase."]
    #[serde(
        rename = "encryption-key",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub encryption_key: Option<String>,
    #[doc = "Certificate SHA 256 fingerprint."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fingerprint: Option<String>,
    #[doc = "Default image format."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub format: Option<String>,
    #[doc = "The Ceph filesystem name."]
    #[serde(rename = "fs-name", skip_serializing_if = "Option::is_none", default)]
    pub fs_name: Option<String>,
    #[doc = "Mount CephFS through FUSE."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub fuse: Option<bool>,
    #[doc = "Assume the given path is an externally managed mountpoint and consider the storage offline if it is not mounted. Using a boolean (yes/no) value serves as a shortcut to using the target path in this field."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub is_mountpoint: Option<String>,
    #[doc = "Client keyring contents (for external clusters)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub keyring: Option<String>,
    #[doc = "Always access rbd through krbd kernel module."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub krbd: Option<bool>,
    #[doc = "target portal group for Linux LIO targets"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lio_tpg: Option<String>,
    #[doc = "Base64-encoded, PEM-formatted public RSA key. Used to encrypt a copy of the encryption-key which will be added to each encrypted backup."]
    #[serde(
        rename = "master-pubkey",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub master_pubkey: Option<String>,
    #[doc = "Maximal number of protected backups per guest. Use '-1' for unlimited."]
    #[serde(
        rename = "max-protected-backups",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_protected_backups: Option<u64>,
    #[doc = "Deprecated: use 'prune-backups' instead. Maximal number of backup files per VM. Use '0' for unlimited."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub maxfiles: Option<u64>,
    #[doc = "Create the directory if it doesn't exist and populate it with default sub-dirs. NOTE: Deprecated, use the 'create-base-path' and 'create-subdirs' options instead."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub mkdir: Option<bool>,
    #[doc = "IP addresses of monitors (for external clusters)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub monhost: Option<String>,
    #[doc = "mount point"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mountpoint: Option<String>,
    #[doc = "Namespace."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub namespace: Option<String>,
    #[doc = "Set the NOCOW flag on files. Disables data checksumming and causes data errors to be unrecoverable from while allowing direct I/O. Only use this if data does not need to be any more safe than on a single ext4 formatted disk with no underlying raid system."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub nocow: Option<bool>,
    #[doc = "List of nodes for which the storage configuration applies."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nodes: Option<String>,
    #[doc = "disable write caching on the target"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub nowritecache: Option<bool>,
    #[doc = "NFS/CIFS mount options (see 'man nfs' or 'man mount.cifs')"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub options: Option<String>,
    #[doc = "Password for accessing the share/datastore."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub password: Option<String>,
    #[doc = "Pool."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pool: Option<String>,
    #[doc = "For non default port."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub port: Option<u64>,
    #[doc = "Preallocation mode for raw and qcow2 images. Using 'metadata' on raw images results in preallocation=off."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub preallocation: Option<String>,
    #[doc = "The retention options with shorter intervals are processed first with --keep-last being the very first one. Each option covers a specific period of time. We say that backups within this period are covered by this option. The next option does not take care of already covered backups and only considers older backups."]
    #[serde(
        rename = "prune-backups",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub prune_backups: Option<String>,
    #[doc = "Zero-out data when removing LVs."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub saferemove: Option<bool>,
    #[doc = "Wipe throughput (cstream -t parameter value)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub saferemove_throughput: Option<String>,
    #[doc = "Server IP or DNS name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub server: Option<String>,
    #[doc = "Backup volfile server IP or DNS name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub server2: Option<String>,
    #[doc = "Indicate that this is a single storage with the same contents on all nodes (or all listed in the 'nodes' option). It will not make the contents of a local storage automatically accessible to other nodes, it just marks an already shared storage as such!"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub shared: Option<bool>,
    #[doc = "Disable TLS certificate verification, only enable on fully trusted networks!"]
    #[serde(
        rename = "skip-cert-verification",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub skip_cert_verification: Option<bool>,
    #[doc = "SMB protocol version. 'default' if not set, negotiates the highest SMB2+ version supported by both the client and server."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub smbversion: Option<String>,
    #[doc = "use sparse volumes"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub sparse: Option<bool>,
    #[doc = "Subdir to mount."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub subdir: Option<String>,
    #[doc = "Only use logical volumes tagged with 'pve-vm-ID'."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub tagged_only: Option<bool>,
    #[doc = "Gluster transport: tcp or rdma"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub transport: Option<String>,
    #[doc = "RBD Id."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutResponseItem {
    #[doc = "Partial, possible server generated, configuration properties."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub config: Option<Config>,
    #[doc = "The ID of the created storage."]
    pub storage: String,
    #[doc = "The type of the created storage."]
    pub r#type: String,
}

#[doc = "Partial, possible server generated, configuration properties."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Config {
    #[doc = "The, possible auto-generated, encryption-key."]
    #[serde(
        rename = "encryption-key",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub encryption_key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StorageClient<T> {
    client: T,
    path: String,
}

impl<T> StorageClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, storage: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, storage),
        }
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete storage configuration."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Read storage configuration."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update storage configuration."]
    pub fn put(&self, parameters: PutParameters) -> Result<PutResponseItem, T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncStorageClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncStorageClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, storage: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, storage),
        }
    }
}
impl<T> AsyncStorageClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete storage configuration."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Read storage configuration."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update storage configuration."]
    pub async fn put(&self, parameters: PutParameters) -> Result<PutResponseItem, T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
