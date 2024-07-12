#[derive(Debug, Clone)]
pub struct ApiversionClient<T> {
    client: T,
    path: String,
}

impl<T> ApiversionClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "apiversion"),
        }
    }
}
impl<T> ApiversionClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Return the version of the cluster join API available on this node."]
    pub fn get(&self) -> Result<u64, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncApiversionClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncApiversionClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "apiversion"),
        }
    }
}
impl<T> AsyncApiversionClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Return the version of the cluster join API available on this node."]
    pub async fn get(&self) -> Result<u64, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
