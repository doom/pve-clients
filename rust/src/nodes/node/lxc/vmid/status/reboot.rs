#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
    #[doc = "Wait maximal timeout seconds for the shutdown."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct RebootClient<T> {
    client: T,
    path: String,
}

impl<T> RebootClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "reboot"),
        }
    }
}
impl<T> RebootClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Reboot the container by shutting it down, and starting it again. Applies pending changes."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncRebootClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncRebootClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "reboot"),
        }
    }
}
impl<T> AsyncRebootClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Reboot the container by shutting it down, and starting it again. Applies pending changes."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
