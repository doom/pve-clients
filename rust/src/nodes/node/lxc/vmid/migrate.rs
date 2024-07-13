#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bwlimit: Option<f64>,
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
    #[doc = "Target node."]
    pub target: String,
    #[doc = "Mapping from source to target storages. Providing only a single storage ID maps all source storages to that storage. Providing the special value '1' will map each source storage to itself."]
    #[serde(
        rename = "target-storage",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub target_storage: Option<String>,
    #[doc = "Timeout in seconds for shutdown for restart migration"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct MigrateClient<T> {
    client: T,
    path: String,
}

impl<T> MigrateClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "migrate"),
        }
    }
}
impl<T> MigrateClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Migrate the container to another node. Creates a new migration task."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMigrateClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMigrateClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "migrate"),
        }
    }
}
impl<T> AsyncMigrateClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Migrate the container to another node. Creates a new migration task."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
