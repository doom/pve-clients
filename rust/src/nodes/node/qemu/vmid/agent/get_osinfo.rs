#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct GetOsinfoClient<T> {
    client: T,
    path: String,
}

impl<T> GetOsinfoClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-osinfo"),
        }
    }
}
impl<T> GetOsinfoClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute get-osinfo."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncGetOsinfoClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncGetOsinfoClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-osinfo"),
        }
    }
}
impl<T> AsyncGetOsinfoClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute get-osinfo."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
