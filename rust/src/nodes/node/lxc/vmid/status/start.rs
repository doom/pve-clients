#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "If set, enables very verbose debug log-level on start."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub debug: Option<bool>,
    #[doc = "Ignore locks - only root is allowed to use this option."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub skiplock: Option<bool>,
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
    #[doc = "Start the container."]
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
    #[doc = "Start the container."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
