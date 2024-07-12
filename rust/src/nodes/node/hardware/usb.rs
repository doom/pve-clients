#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub busnum: u64,
    pub class: u64,
    pub devnum: u64,
    pub level: u64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub manufacturer: Option<String>,
    pub port: u64,
    pub prodid: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub serial: Option<String>,
    pub speed: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub usbpath: Option<String>,
    pub vendid: String,
}

#[derive(Debug, Clone)]
pub struct UsbClient<T> {
    client: T,
    path: String,
}

impl<T> UsbClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "usb"),
        }
    }
}
impl<T> UsbClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List local USB devices."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncUsbClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncUsbClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "usb"),
        }
    }
}
impl<T> AsyncUsbClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List local USB devices."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
