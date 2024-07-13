#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Maximal number of parallel migration job. If not set, uses'max_workers' from datacenter.cfg. One of both must be set!"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub maxworkers: Option<u64>,
    #[doc = "Target node."]
    pub target: String,
    #[doc = "Only consider Guests with these IDs."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vms: Option<String>,
    #[doc = "Enable live storage migration for local disk"]
    #[serde(
        rename = "with-local-disks",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub with_local_disks: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct MigrateallClient<T> {
    client: T,
    path: String,
}

impl<T> MigrateallClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "migrateall"),
        }
    }
}
impl<T> MigrateallClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Migrate all VMs and Containers."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMigrateallClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMigrateallClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "migrateall"),
        }
    }
}
impl<T> AsyncMigrateallClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Migrate all VMs and Containers."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
