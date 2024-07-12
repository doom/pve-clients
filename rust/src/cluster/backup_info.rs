pub mod not_backed_up;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "API sub-directory endpoint"]
    pub subdir: String,
}

#[derive(Debug, Clone)]
pub struct BackupInfoClient<T> {
    client: T,
    path: String,
}

impl<T> BackupInfoClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "backup-info"),
        }
    }

    pub fn not_backed_up(&self) -> not_backed_up::NotBackedUpClient<T> {
        not_backed_up::NotBackedUpClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> BackupInfoClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Index for backup info related endpoints"]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncBackupInfoClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncBackupInfoClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "backup-info"),
        }
    }

    pub fn not_backed_up(&self) -> not_backed_up::AsyncNotBackedUpClient<T> {
        not_backed_up::AsyncNotBackedUpClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncBackupInfoClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Index for backup info related endpoints"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
