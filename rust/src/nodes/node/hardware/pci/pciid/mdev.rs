#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The number of still available instances of this type."]
    pub available: u64,
    pub description: String,
    #[doc = "The name of the mdev type."]
    pub r#type: String,
}

#[derive(Debug, Clone)]
pub struct MdevClient<T> {
    client: T,
    path: String,
}

impl<T> MdevClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "mdev"),
        }
    }
}
impl<T> MdevClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List mediated device types for given PCI device."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMdevClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMdevClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "mdev"),
        }
    }
}
impl<T> AsyncMdevClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List mediated device types for given PCI device."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
