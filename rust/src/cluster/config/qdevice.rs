#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct QdeviceClient<T> {
    client: T,
    path: String,
}

impl<T> QdeviceClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "qdevice"),
        }
    }
}
impl<T> QdeviceClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get QDevice status"]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncQdeviceClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncQdeviceClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "qdevice"),
        }
    }
}
impl<T> AsyncQdeviceClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get QDevice status"]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
