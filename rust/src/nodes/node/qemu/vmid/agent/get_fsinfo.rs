#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct GetFsinfoClient<T> {
    client: T,
    path: String,
}

impl<T> GetFsinfoClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-fsinfo"),
        }
    }
}
impl<T> GetFsinfoClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute get-fsinfo."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncGetFsinfoClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncGetFsinfoClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "get-fsinfo"),
        }
    }
}
impl<T> AsyncGetFsinfoClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute get-fsinfo."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
