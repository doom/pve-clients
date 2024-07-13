pub mod pci;
pub mod usb;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub r#type: String,
}

#[derive(Debug, Clone)]
pub struct HardwareClient<T> {
    client: T,
    path: String,
}

impl<T> HardwareClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "hardware"),
        }
    }

    pub fn pci(&self) -> pci::PciClient<T> {
        pci::PciClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn usb(&self) -> usb::UsbClient<T> {
        usb::UsbClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> HardwareClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Index of hardware types"]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncHardwareClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncHardwareClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "hardware"),
        }
    }

    pub fn pci(&self) -> pci::AsyncPciClient<T> {
        pci::AsyncPciClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn usb(&self) -> usb::AsyncUsbClient<T> {
        usb::AsyncUsbClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncHardwareClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Index of hardware types"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
