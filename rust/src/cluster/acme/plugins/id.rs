#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "API plugin name"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub api: Option<String>,
    #[doc = "DNS plugin data. (base64 encoded)"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub data: Option<String>,
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Flag to disable the config."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub disable: Option<bool>,
    #[doc = "List of cluster node names."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nodes: Option<String>,
    #[doc = "Extra delay in seconds to wait before requesting validation. Allows to cope with a long TTL of DNS records."]
    #[serde(
        rename = "validation-delay",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub validation_delay: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct IdClient<T> {
    client: T,
    path: String,
}

impl<T> IdClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, id),
        }
    }
}
impl<T> IdClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete ACME plugin configuration."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Get ACME plugin configuration."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update ACME plugin configuration."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncIdClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncIdClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, id),
        }
    }
}
impl<T> AsyncIdClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete ACME plugin configuration."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Get ACME plugin configuration."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update ACME plugin configuration."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
