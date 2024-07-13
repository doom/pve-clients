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
    #[doc = "Creates a TCP proxy connection."]
    pub fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &())
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
    #[doc = "Creates a TCP proxy connection."]
    pub async fn post(&self) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
