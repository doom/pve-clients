#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub scope: Option<String>,
}

#[doc = "Items for each type of service containing objects for each instance."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Metadata servers configured in the cluster and their properties."]
    pub mds: Mds,
    #[doc = "Managers configured in the cluster and their properties."]
    pub mgr: Mgr,
    #[doc = "Monitors configured in the cluster and their properties."]
    pub mon: Mon,
    #[doc = "Ceph version installed on the nodes."]
    pub node: Node,
    #[doc = "OSDs configured in the cluster and their properties."]
    pub osd: Vec<serde_json::Value>,
}

#[doc = "Metadata servers configured in the cluster and their properties."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Mds {}

#[doc = "Managers configured in the cluster and their properties."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Mgr {}

#[doc = "Monitors configured in the cluster and their properties."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Mon {}

#[doc = "Ceph version installed on the nodes."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Node {}

#[derive(Debug, Clone)]
pub struct MetadataClient<T> {
    client: T,
    path: String,
}

impl<T> MetadataClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "metadata"),
        }
    }
}
impl<T> MetadataClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get ceph metadata."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMetadataClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMetadataClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "metadata"),
        }
    }
}
impl<T> AsyncMetadataClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get ceph metadata."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
