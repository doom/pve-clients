#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "The (unique) ID of the VM."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmid: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct NextidClient<T> {
    client: T,
    path: String,
}

impl<T> NextidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "nextid"),
        }
    }
}
impl<T> NextidClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get next free VMID. Pass a VMID to assert that its free (at time of check)."]
    pub fn get(&self, parameters: GetParameters) -> Result<u64, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncNextidClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNextidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "nextid"),
        }
    }
}
impl<T> AsyncNextidClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get next free VMID. Pass a VMID to assert that its free (at time of check)."]
    pub async fn get(&self, parameters: GetParameters) -> Result<u64, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
