#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
    #[doc = "Ceph service name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub service: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RestartClient<T> {
    client: T,
    path: String,
}

impl<T> RestartClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "restart"),
        }
    }
}
impl<T> RestartClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Restart ceph services."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncRestartClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncRestartClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "restart"),
        }
    }
}
impl<T> AsyncRestartClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Restart ceph services."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
