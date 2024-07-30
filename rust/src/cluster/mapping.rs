pub mod pci;
pub mod usb;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct MappingClient<T> {
    client: T,
    path: String,
}

impl<T> MappingClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "mapping"),
        }
    }

    pub fn pci(&self) -> pci::PciClient<T> {
        pci::PciClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn usb(&self) -> usb::UsbClient<T> {
        usb::UsbClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> MappingClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List resource types."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMappingClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMappingClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "mapping"),
        }
    }

    pub fn pci(&self) -> pci::AsyncPciClient<T> {
        pci::AsyncPciClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn usb(&self) -> usb::AsyncUsbClient<T> {
        usb::AsyncUsbClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncMappingClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List resource types."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
