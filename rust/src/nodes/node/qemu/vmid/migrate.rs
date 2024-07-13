#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "Target node."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "List nodes allowed for offline migration, only passed if VM is offline"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub allowed_nodes: Option<Vec<serde_json::Value>>,
    #[doc = "List local disks including CD-Rom, unsused and not referenced disks"]
    pub local_disks: Vec<serde_json::Value>,
    #[doc = "List local resources e.g. pci, usb"]
    pub local_resources: Vec<serde_json::Value>,
    #[doc = "List not allowed nodes with additional informations, only passed if VM is offline"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub not_allowed_nodes: Option<NotAllowedNodes>,
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub running: bool,
}

#[doc = "List not allowed nodes with additional informations, only passed if VM is offline"]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotAllowedNodes {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bwlimit: Option<u64>,
    #[doc = "Allow to migrate VMs which use local devices. Only root may use this option."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub force: Option<bool>,
    #[doc = "CIDR of the (sub) network that is used for migration."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub migration_network: Option<String>,
    #[doc = "Migration traffic is encrypted using an SSH tunnel by default. On secure, completely private networks this can be disabled to increase performance."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub migration_type: Option<String>,
    #[doc = "Use online/live migration if VM is running. Ignored if VM is stopped."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub online: Option<bool>,
    #[doc = "Target node."]
    pub target: String,
    #[doc = "Mapping from source to target storages. Providing only a single storage ID maps all source storages to that storage. Providing the special value '1' will map each source storage to itself."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub targetstorage: Option<String>,
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
    #[doc = "Get preconditions for migration."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Migrate virtual machine. Creates a new migration task."]
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
    #[doc = "Get preconditions for migration."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Migrate virtual machine. Creates a new migration task."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
