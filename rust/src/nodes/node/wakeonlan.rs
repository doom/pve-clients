#[derive(Debug, Clone)]
pub struct WakeonlanClient<T> {
    client: T,
    path: String,
}

impl<T> WakeonlanClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "wakeonlan"),
        }
    }
}
impl<T> WakeonlanClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Try to wake a node via 'wake on LAN' network packet."]
    pub fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncWakeonlanClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncWakeonlanClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "wakeonlan"),
        }
    }
}
impl<T> AsyncWakeonlanClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Try to wake a node via 'wake on LAN' network packet."]
    pub async fn post(&self) -> Result<String, T::Error> {
        self.client.post(&self.path, &()).await
    }
}
