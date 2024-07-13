pub mod mdev;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub method: String,
}

#[derive(Debug, Clone)]
pub struct PciidClient<T> {
    client: T,
    path: String,
}

impl<T> PciidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, pciid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, pciid),
        }
    }

    pub fn mdev(&self) -> mdev::MdevClient<T> {
        mdev::MdevClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> PciidClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Index of available pci methods"]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncPciidClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncPciidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, pciid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, pciid),
        }
    }

    pub fn mdev(&self) -> mdev::AsyncMdevClient<T> {
        mdev::AsyncMdevClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncPciidClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Index of available pci methods"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
