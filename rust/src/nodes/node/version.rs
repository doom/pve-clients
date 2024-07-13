#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The current installed Proxmox VE Release"]
    pub release: String,
    #[doc = "The short git commit hash ID from which this version was build"]
    pub repoid: String,
    #[doc = "The current installed pve-manager package version"]
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct VersionClient<T> {
    client: T,
    path: String,
}

impl<T> VersionClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "version"),
        }
    }
}
impl<T> VersionClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "API version details"]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncVersionClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncVersionClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "version"),
        }
    }
}
impl<T> AsyncVersionClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "API version details"]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
