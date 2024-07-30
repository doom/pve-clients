#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
    #[doc = "Send notification about new packages."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub notify: Option<bool>,
    #[doc = "Only produces output suitable for logging, omitting progress indicators."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub quiet: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct UpdateClient<T> {
    client: T,
    path: String,
}

impl<T> UpdateClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "update"),
        }
    }
}
impl<T> UpdateClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List available updates."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "This is used to resynchronize the package index files from their sources (apt-get update)."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncUpdateClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncUpdateClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "update"),
        }
    }
}
impl<T> AsyncUpdateClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List available updates."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "This is used to resynchronize the package index files from their sources (apt-get update)."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
