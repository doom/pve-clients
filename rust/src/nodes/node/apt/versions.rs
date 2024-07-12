#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct VersionsClient<T> {
    client: T,
    path: String,
}

impl<T> VersionsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "versions"),
        }
    }
}
impl<T> VersionsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get package information for important Proxmox packages."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncVersionsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncVersionsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "versions"),
        }
    }
}
impl<T> AsyncVersionsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get package information for important Proxmox packages."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
