#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct TotemClient<T> {
    client: T,
    path: String,
}

impl<T> TotemClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "totem"),
        }
    }
}
impl<T> TotemClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get corosync totem protocol settings."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncTotemClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncTotemClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "totem"),
        }
    }
}
impl<T> AsyncTotemClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get corosync totem protocol settings."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
