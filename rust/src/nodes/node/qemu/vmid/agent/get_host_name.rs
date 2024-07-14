#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct GetHostNameClient<T> {
    client: T,
    path: String,
}

impl<T> GetHostNameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-host-name"),
        }
    }
}
impl<T> GetHostNameClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute get-host-name."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncGetHostNameClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncGetHostNameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-host-name"),
        }
    }
}
impl<T> AsyncGetHostNameClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute get-host-name."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
