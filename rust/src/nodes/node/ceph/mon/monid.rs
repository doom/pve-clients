#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Overwrites autodetected monitor IP address(es). Must be in the public network(s) of Ceph."]
    #[serde(
        rename = "mon-address",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub mon_address: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MonidClient<T> {
    client: T,
    path: String,
}

impl<T> MonidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, monid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, monid),
        }
    }
}
impl<T> MonidClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Destroy Ceph Monitor and Manager."]
    pub fn delete(&self) -> Result<String, T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Create Ceph Monitor and Manager"]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMonidClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMonidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, monid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, monid),
        }
    }
}
impl<T> AsyncMonidClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Destroy Ceph Monitor and Manager."]
    pub async fn delete(&self) -> Result<String, T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Create Ceph Monitor and Manager"]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
