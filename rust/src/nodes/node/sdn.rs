pub mod zones;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct SdnClient<T> {
    client: T,
    path: String,
}

impl<T> SdnClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "sdn"),
        }
    }

    pub fn zones(&self) -> zones::ZonesClient<T> {
        zones::ZonesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> SdnClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "SDN index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSdnClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSdnClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "sdn"),
        }
    }

    pub fn zones(&self) -> zones::AsyncZonesClient<T> {
        zones::AsyncZonesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncSdnClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "SDN index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
