#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Make sure the Container stops."]
    #[serde(
        rename = "forceStop",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub force_stop: Option<bool>,
    #[doc = "Wait maximal timeout seconds."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct ShutdownClient<T> {
    client: T,
    path: String,
}

impl<T> ShutdownClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "shutdown"),
        }
    }
}
impl<T> ShutdownClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Shutdown the container. This will trigger a clean shutdown of the container, see lxc-stop(1) for details."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncShutdownClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncShutdownClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "shutdown"),
        }
    }
}
impl<T> AsyncShutdownClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Shutdown the container. This will trigger a clean shutdown of the container, see lxc-stop(1) for details."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
