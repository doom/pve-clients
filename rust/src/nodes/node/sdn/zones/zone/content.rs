#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Status."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub status: Option<String>,
    #[doc = "Status details"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub statusmsg: Option<String>,
    #[doc = "Vnet identifier."]
    pub vnet: String,
}

#[derive(Debug, Clone)]
pub struct ContentClient<T> {
    client: T,
    path: String,
}

impl<T> ContentClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "content"),
        }
    }
}
impl<T> ContentClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List zone content."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncContentClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncContentClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "content"),
        }
    }
}
impl<T> AsyncContentClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List zone content."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
