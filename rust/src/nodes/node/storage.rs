pub mod storage;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "Only list stores which support this content type."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub content: Option<String>,
    #[doc = "Only list stores which are enabled (not disabled in config)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enabled: Option<bool>,
    #[doc = "Include information about formats"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub format: Option<bool>,
    #[doc = "Only list status for  specified storage"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub storage: Option<String>,
    #[doc = "If target is different to 'node', we only lists shared storages which content is accessible on this 'node' and the specified 'target' node."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Set when storage is accessible."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub active: Option<bool>,
    #[doc = "Available storage space in bytes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub avail: Option<u64>,
    #[doc = "Allowed storage content types."]
    pub content: String,
    #[doc = "Set when storage is enabled (not disabled)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enabled: Option<bool>,
    #[doc = "Shared flag from storage configuration."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub shared: Option<bool>,
    #[doc = "The storage identifier."]
    pub storage: String,
    #[doc = "Total storage space in bytes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub total: Option<u64>,
    #[doc = "Storage type."]
    pub r#type: String,
    #[doc = "Used storage space in bytes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub used: Option<u64>,
    #[doc = "Used fraction (used/total)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub used_fraction: Option<f64>,
}

impl GetResponseItem {
    pub fn new(content: String, storage: String, r#type: String) -> Self {
        Self {
            content,
            storage,
            r#type,
            active: Default::default(),
            avail: Default::default(),
            enabled: Default::default(),
            shared: Default::default(),
            total: Default::default(),
            used: Default::default(),
            used_fraction: Default::default(),
        }
    }
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
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "storage"),
        }
    }

    pub fn storage(&self, storage: &str) -> storage::StorageClient<T> {
        storage::StorageClient::<T>::new(self.client.clone(), &self.path, storage)
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get status for all datastores."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
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
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "storage"),
        }
    }

    pub fn storage(&self, storage: &str) -> storage::AsyncStorageClient<T> {
        storage::AsyncStorageClient::<T>::new(self.client.clone(), &self.path, storage)
    }
}
impl<T> AsyncStorageClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get status for all datastores."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
