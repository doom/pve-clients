#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Maximum usable CPUs."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cpus: Option<f64>,
    #[doc = "HA manager service status."]
    pub ha: Ha,
    #[doc = "The current config lock, if any."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lock: Option<String>,
    #[doc = "Root disk size in bytes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub maxdisk: Option<u64>,
    #[doc = "Maximum memory in bytes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub maxmem: Option<u64>,
    #[doc = "Maximum SWAP memory in bytes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub maxswap: Option<u64>,
    #[doc = "Container name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    #[doc = "LXC Container status."]
    pub status: String,
    #[doc = "The current configured tags, if any."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<String>,
    #[doc = "Uptime."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub uptime: Option<u64>,
    #[doc = "The (unique) ID of the VM."]
    pub vmid: u64,
}

#[doc = "HA manager service status."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Ha {}

#[derive(Debug, Clone)]
pub struct CurrentClient<T> {
    client: T,
    path: String,
}

impl<T> CurrentClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "current"),
        }
    }
}
impl<T> CurrentClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get virtual machine status."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCurrentClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCurrentClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "current"),
        }
    }
}
impl<T> AsyncCurrentClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get virtual machine status."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
