#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct InfoClient<T> {
    client: T,
    path: String,
}

impl<T> InfoClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "info"),
        }
    }
}
impl<T> InfoClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute info."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncInfoClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncInfoClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "info"),
        }
    }
}
impl<T> AsyncInfoClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute info."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
