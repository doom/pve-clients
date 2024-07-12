#[derive(Debug, Clone)]
pub struct OutClient<T> {
    client: T,
    path: String,
}

impl<T> OutClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "out"),
        }
    }
}
impl<T> OutClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "ceph osd out"]
    pub fn post(&self) -> Result<(), T::Error> {
        self.client.post(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncOutClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncOutClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "out"),
        }
    }
}
impl<T> AsyncOutClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "ceph osd out"]
    pub async fn post(&self) -> Result<(), T::Error> {
        self.client.post(&self.path, &()).await
    }
}
