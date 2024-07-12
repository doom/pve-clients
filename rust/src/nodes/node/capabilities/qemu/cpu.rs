#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "True if this is a custom CPU model."]
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub custom: bool,
    #[doc = "Name of the CPU model. Identifies it for subsequent API calls. Prefixed with 'custom-' for custom models."]
    pub name: String,
    #[doc = "CPU vendor visible to the guest when this model is selected. Vendor of 'reported-model' in case of custom models."]
    pub vendor: String,
}

#[derive(Debug, Clone)]
pub struct CpuClient<T> {
    client: T,
    path: String,
}

impl<T> CpuClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "cpu"),
        }
    }
}
impl<T> CpuClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List all custom and default CPU models."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCpuClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCpuClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "cpu"),
        }
    }
}
impl<T> AsyncCpuClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List all custom and default CPU models."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
