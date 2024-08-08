#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Comment"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Disable this target"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub disable: Option<bool>,
    #[doc = "The name of the endpoint."]
    pub name: String,
    #[doc = "Server URL"]
    pub server: String,
}

impl GetResponseItem {
    pub fn new(name: String, server: String) -> Self {
        Self {
            name,
            server,
            comment: Default::default(),
            digest: Default::default(),
            disable: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "Comment"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<Vec<String>>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Disable this target"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub disable: Option<bool>,
    #[doc = "Server URL"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub server: Option<String>,
    #[doc = "Secret token"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NameClient<T> {
    client: T,
    path: String,
}

impl<T> NameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
impl<T> NameClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Remove gotify endpoint"]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Return a specific gotify endpoint"]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update existing gotify endpoint"]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncNameClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
impl<T> AsyncNameClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Remove gotify endpoint"]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Return a specific gotify endpoint"]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update existing gotify endpoint"]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
