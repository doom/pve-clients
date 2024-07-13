#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The default console viewer to use."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub console: Option<String>,
    #[doc = "The current Proxmox VE point release in `x.y` format."]
    pub release: String,
    #[doc = "The short git revision from which this version was build."]
    pub repoid: String,
    #[doc = "The full pve-manager package version of this node."]
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
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "version".to_string(),
        }
    }
}
impl<T> VersionClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "API version details, including some parts of the global datacenter config."]
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
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "version".to_string(),
        }
    }
}
impl<T> AsyncVersionClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "API version details, including some parts of the global datacenter config."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
