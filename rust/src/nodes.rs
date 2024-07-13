pub mod node;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "CPU utilization."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cpu: Option<f64>,
    #[doc = "Support level."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub level: Option<String>,
    #[doc = "Number of available CPUs."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub maxcpu: Option<u64>,
    #[doc = "Number of available memory in bytes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub maxmem: Option<u64>,
    #[doc = "Used memory in bytes."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mem: Option<u64>,
    #[doc = "The cluster node name."]
    pub node: String,
    #[doc = "The SSL fingerprint for the node certificate."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ssl_fingerprint: Option<String>,
    #[doc = "Node status."]
    pub status: String,
    #[doc = "Node uptime in seconds."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub uptime: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct NodesClient<T> {
    client: T,
    path: String,
}

impl<T> NodesClient<T>
where
    T: Clone,
{
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "nodes".to_string(),
        }
    }

    pub fn node(&self, node: &str) -> node::NodeClient<T> {
        node::NodeClient::<T>::new(self.client.clone(), &self.path, node)
    }
}
impl<T> NodesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Cluster node index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncNodesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNodesClient<T>
where
    T: Clone,
{
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "nodes".to_string(),
        }
    }

    pub fn node(&self, node: &str) -> node::AsyncNodeClient<T> {
        node::AsyncNodeClient::<T>::new(self.client.clone(), &self.path, node)
    }
}
impl<T> AsyncNodesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Cluster node index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
