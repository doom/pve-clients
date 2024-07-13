#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "The new value of the flag"]
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub value: bool,
}

#[derive(Debug, Clone)]
pub struct FlagClient<T> {
    client: T,
    path: String,
}

impl<T> FlagClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, flag: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, flag),
        }
    }
}
impl<T> FlagClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get the status of a specific ceph flag."]
    pub fn get(&self) -> Result<bool, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Set or clear (unset) a specific ceph flag"]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncFlagClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncFlagClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, flag: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, flag),
        }
    }
}
impl<T> AsyncFlagClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get the status of a specific ceph flag."]
    pub async fn get(&self) -> Result<bool, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Set or clear (unset) a specific ceph flag"]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
