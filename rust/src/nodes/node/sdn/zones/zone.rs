pub mod content;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub subdir: String,
}

#[derive(Debug, Clone)]
pub struct ZoneClient<T> {
    client: T,
    path: String,
}

impl<T> ZoneClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, zone: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, zone),
        }
    }

    pub fn content(&self) -> content::ContentClient<T> {
        content::ContentClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ZoneClient<T>
where
    T: crate::client::HttpClient,
{
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncZoneClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncZoneClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, zone: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, zone),
        }
    }

    pub fn content(&self) -> content::AsyncContentClient<T> {
        content::AsyncContentClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncZoneClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
