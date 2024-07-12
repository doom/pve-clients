pub mod id;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "Only list replication jobs for this guest."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub guest: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct ReplicationClient<T> {
    client: T,
    path: String,
}

impl<T> ReplicationClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "replication"),
        }
    }

    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> ReplicationClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List status of all replication jobs on this node."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncReplicationClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncReplicationClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "replication"),
        }
    }

    pub fn id(&self, id: &str) -> id::AsyncIdClient<T> {
        id::AsyncIdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> AsyncReplicationClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List status of all replication jobs on this node."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
