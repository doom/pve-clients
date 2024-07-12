#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
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
pub struct StopClient<T> {
    client: T,
    path: String,
}

impl<T> StopClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "stop"),
        }
    }
}
impl<T> StopClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Stop the container. This will abruptly stop all processes running in the container."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncStopClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncStopClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "stop"),
        }
    }
}
impl<T> AsyncStopClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Stop the container. This will abruptly stop all processes running in the container."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
