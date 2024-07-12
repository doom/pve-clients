pub mod node;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub node: String,
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
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "nodes"),
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
    #[doc = "Corosync node list."]
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
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "nodes"),
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
    #[doc = "Corosync node list."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
