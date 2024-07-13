#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct NetstatClient<T> {
    client: T,
    path: String,
}

impl<T> NetstatClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "netstat"),
        }
    }
}
impl<T> NetstatClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Read tap/vm network device interface counters"]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncNetstatClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNetstatClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "netstat"),
        }
    }
}
impl<T> AsyncNetstatClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Read tap/vm network device interface counters"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
