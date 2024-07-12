#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "ZFS pool name."]
    pub pool: String,
}

#[derive(Debug, Clone)]
pub struct ZfsClient<T> {
    client: T,
    path: String,
}

impl<T> ZfsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "zfs"),
        }
    }
}
impl<T> ZfsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Scan zfs pool list on local node."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncZfsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncZfsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "zfs"),
        }
    }
}
impl<T> AsyncZfsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Scan zfs pool list on local node."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
