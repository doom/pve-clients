#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "opens a serial terminal (defaults to display)"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub serial: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {
    pub port: u64,
    pub ticket: String,
    pub upid: String,
    pub user: String,
}

#[derive(Debug, Clone)]
pub struct TermproxyClient<T> {
    client: T,
    path: String,
}

impl<T> TermproxyClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "termproxy"),
        }
    }
}
impl<T> TermproxyClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Creates a TCP proxy connections."]
    pub fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncTermproxyClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncTermproxyClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "termproxy"),
        }
    }
}
impl<T> AsyncTermproxyClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Creates a TCP proxy connections."]
    pub async fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
