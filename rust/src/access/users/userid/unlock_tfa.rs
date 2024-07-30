#[derive(Debug, Clone)]
pub struct UnlockTfaClient<T> {
    client: T,
    path: String,
}

impl<T> UnlockTfaClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "unlock-tfa"),
        }
    }
}
impl<T> UnlockTfaClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Unlock a user's TFA authentication."]
    pub fn put(&self) -> Result<bool, T::Error> {
        self.client.put(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncUnlockTfaClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncUnlockTfaClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "unlock-tfa"),
        }
    }
}
impl<T> AsyncUnlockTfaClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Unlock a user's TFA authentication."]
    pub async fn put(&self) -> Result<bool, T::Error> {
        self.client.put(&self.path, &()).await
    }
}
