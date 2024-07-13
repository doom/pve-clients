#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bwlimit: Option<f64>,
    #[doc = "Delete the original CT and related data after successful migration. By default the original CT is kept on the source cluster in a stopped state."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub delete: Option<bool>,
    #[doc = "Use online/live migration."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub online: Option<bool>,
    #[doc = "Use restart migration"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub restart: Option<bool>,
    #[doc = "Mapping from source to target bridges. Providing only a single bridge ID maps all source bridges to that bridge. Providing the special value '1' will map each source bridge to itself."]
    #[serde(rename = "target-bridge")]
    pub target_bridge: String,
    #[doc = "Remote target endpoint"]
    #[serde(rename = "target-endpoint")]
    pub target_endpoint: String,
    #[doc = "Mapping from source to target storages. Providing only a single storage ID maps all source storages to that storage. Providing the special value '1' will map each source storage to itself."]
    #[serde(rename = "target-storage")]
    pub target_storage: String,
    #[doc = "The (unique) ID of the VM."]
    #[serde(
        rename = "target-vmid",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub target_vmid: Option<u64>,
    #[doc = "Timeout in seconds for shutdown for restart migration"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct RemoteMigrateClient<T> {
    client: T,
    path: String,
}

impl<T> RemoteMigrateClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "remote_migrate"),
        }
    }
}
impl<T> RemoteMigrateClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Migrate the container to another cluster. Creates a new migration task. EXPERIMENTAL feature!"]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncRemoteMigrateClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncRemoteMigrateClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "remote_migrate"),
        }
    }
}
impl<T> AsyncRemoteMigrateClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Migrate the container to another cluster. Creates a new migration task. EXPERIMENTAL feature!"]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
