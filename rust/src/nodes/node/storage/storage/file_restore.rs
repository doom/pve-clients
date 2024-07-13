pub mod download;
pub mod list;

#[derive(Debug, Clone)]
pub struct FileRestoreClient<T> {
    client: T,
    path: String,
}

impl<T> FileRestoreClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "file-restore"),
        }
    }

    pub fn list(&self) -> list::ListClient<T> {
        list::ListClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn download(&self) -> download::DownloadClient<T> {
        download::DownloadClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> FileRestoreClient<T> where T: crate::client::HttpClient {}
#[derive(Debug, Clone)]
pub struct AsyncFileRestoreClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncFileRestoreClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "file-restore"),
        }
    }

    pub fn list(&self) -> list::AsyncListClient<T> {
        list::AsyncListClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn download(&self) -> download::AsyncDownloadClient<T> {
        download::AsyncDownloadClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncFileRestoreClient<T> where T: crate::client::AsyncHttpClient {}
