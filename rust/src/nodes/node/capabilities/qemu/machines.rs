#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Full name of machine type and version."]
    pub id: String,
    #[doc = "The machine type."]
    pub r#type: String,
    #[doc = "The machine version."]
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct MachinesClient<T> {
    client: T,
    path: String,
}

impl<T> MachinesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "machines"),
        }
    }
}
impl<T> MachinesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get available QEMU/KVM machine types."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMachinesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMachinesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "machines"),
        }
    }
}
impl<T> AsyncMachinesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get available QEMU/KVM machine types."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
