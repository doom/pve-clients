#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Determines whether a ceph-mds daemon should poll and replay the log of an active MDS. Faster switch on MDS failure, but needs more idle resources."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub hotstandby: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct NameClient<T> {
    client: T,
    path: String,
}

impl<T> NameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
impl<T> NameClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Destroy Ceph Metadata Server"]
    pub fn delete(&self) -> Result<String, T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Create Ceph Metadata Server (MDS)"]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncNameClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
impl<T> AsyncNameClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Destroy Ceph Metadata Server"]
    pub async fn delete(&self) -> Result<String, T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Create Ceph Metadata Server (MDS)"]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
