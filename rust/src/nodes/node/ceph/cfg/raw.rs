#[derive(Debug, Clone)]
pub struct RawClient<T> {
    client: T,
    path: String,
}

impl<T> RawClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "raw"),
        }
    }
}
impl<T> RawClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get the Ceph configuration file."]
    pub fn get(&self) -> Result<String, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncRawClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncRawClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "raw"),
        }
    }
}
impl<T> AsyncRawClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get the Ceph configuration file."]
    pub async fn get(&self) -> Result<String, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
