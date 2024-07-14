pub mod qemu;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct CapabilitiesClient<T> {
    client: T,
    path: String,
}

impl<T> CapabilitiesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "capabilities"),
        }
    }

    pub fn qemu(&self) -> qemu::QemuClient<T> {
        qemu::QemuClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CapabilitiesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Node capabilities index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCapabilitiesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCapabilitiesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "capabilities"),
        }
    }

    pub fn qemu(&self) -> qemu::AsyncQemuClient<T> {
        qemu::AsyncQemuClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncCapabilitiesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Node capabilities index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
