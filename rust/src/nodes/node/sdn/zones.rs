pub mod zone;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Status of zone"]
    pub status: String,
    #[doc = "The SDN zone object identifier."]
    pub zone: String,
}

#[derive(Debug, Clone)]
pub struct ZonesClient<T> {
    client: T,
    path: String,
}

impl<T> ZonesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "zones"),
        }
    }

    pub fn zone(&self, zone: &str) -> zone::ZoneClient<T> {
        zone::ZoneClient::<T>::new(self.client.clone(), &self.path, zone)
    }
}
impl<T> ZonesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get status for all zones."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncZonesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncZonesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "zones"),
        }
    }

    pub fn zone(&self, zone: &str) -> zone::AsyncZoneClient<T> {
        zone::AsyncZoneClient::<T>::new(self.client.clone(), &self.path, zone)
    }
}
impl<T> AsyncZonesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get status for all zones."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
