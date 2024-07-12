#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Override QEMU's -cpu argument with the given string."]
    #[serde(rename = "force-cpu", skip_serializing_if = "Option::is_none", default)]
    pub force_cpu: Option<String>,
    #[doc = "Specifies the QEMU machine type."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub machine: Option<String>,
    #[doc = "The cluster node name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub migratedfrom: Option<String>,
    #[doc = "CIDR of the (sub) network that is used for migration."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub migration_network: Option<String>,
    #[doc = "Migration traffic is encrypted using an SSH tunnel by default. On secure, completely private networks this can be disabled to increase performance."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub migration_type: Option<String>,
    #[doc = "Ignore locks - only root is allowed to use this option."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub skiplock: Option<bool>,
    #[doc = "Some command save/restore state from this location."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub stateuri: Option<String>,
    #[doc = "Mapping from source to target storages. Providing only a single storage ID maps all source storages to that storage. Providing the special value '1' will map each source storage to itself."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub targetstorage: Option<String>,
    #[doc = "Wait maximal timeout seconds."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct StartClient<T> {
    client: T,
    path: String,
}

impl<T> StartClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "start"),
        }
    }
}
impl<T> StartClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Start virtual machine."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncStartClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncStartClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "start"),
        }
    }
}
impl<T> AsyncStartClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Start virtual machine."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
