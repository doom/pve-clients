pub mod id;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "Only list ACME plugins of a specific type"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Unique identifier for ACME plugin instance."]
    pub plugin: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "API plugin name"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub api: Option<String>,
    #[doc = "DNS plugin data. (base64 encoded)"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub data: Option<String>,
    #[doc = "Flag to disable the config."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub disable: Option<bool>,
    #[doc = "ACME Plugin ID name"]
    pub id: String,
    #[doc = "List of cluster node names."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nodes: Option<String>,
    #[doc = "ACME challenge type."]
    pub r#type: String,
    #[doc = "Extra delay in seconds to wait before requesting validation. Allows to cope with a long TTL of DNS records."]
    #[serde(
        rename = "validation-delay",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub validation_delay: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct PluginsClient<T> {
    client: T,
    path: String,
}

impl<T> PluginsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "plugins"),
        }
    }

    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> PluginsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "ACME plugin index."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Add ACME plugin configuration."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncPluginsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncPluginsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "plugins"),
        }
    }

    pub fn id(&self, id: &str) -> id::AsyncIdClient<T> {
        id::AsyncIdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> AsyncPluginsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "ACME plugin index."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Add ACME plugin configuration."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
