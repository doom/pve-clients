pub mod content;
pub mod download_url;
pub mod file_restore;
pub mod prunebackups;
pub mod rrd;
pub mod rrddata;
pub mod status;
pub mod upload;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub subdir: String,
}

#[derive(Debug, Clone)]
pub struct StorageClient<T> {
    client: T,
    path: String,
}

impl<T> StorageClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, storage: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, storage),
        }
    }

    pub fn prunebackups(&self) -> prunebackups::PrunebackupsClient<T> {
        prunebackups::PrunebackupsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn content(&self) -> content::ContentClient<T> {
        content::ContentClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn file_restore(&self) -> file_restore::FileRestoreClient<T> {
        file_restore::FileRestoreClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rrd(&self) -> rrd::RrdClient<T> {
        rrd::RrdClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rrddata(&self) -> rrddata::RrddataClient<T> {
        rrddata::RrddataClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn upload(&self) -> upload::UploadClient<T> {
        upload::UploadClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn download_url(&self) -> download_url::DownloadUrlClient<T> {
        download_url::DownloadUrlClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::HttpClient,
{
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncStorageClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncStorageClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, storage: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, storage),
        }
    }

    pub fn prunebackups(&self) -> prunebackups::AsyncPrunebackupsClient<T> {
        prunebackups::AsyncPrunebackupsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn content(&self) -> content::AsyncContentClient<T> {
        content::AsyncContentClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn file_restore(&self) -> file_restore::AsyncFileRestoreClient<T> {
        file_restore::AsyncFileRestoreClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn status(&self) -> status::AsyncStatusClient<T> {
        status::AsyncStatusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rrd(&self) -> rrd::AsyncRrdClient<T> {
        rrd::AsyncRrdClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rrddata(&self) -> rrddata::AsyncRrddataClient<T> {
        rrddata::AsyncRrddataClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn upload(&self) -> upload::AsyncUploadClient<T> {
        upload::AsyncUploadClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn download_url(&self) -> download_url::AsyncDownloadUrlClient<T> {
        download_url::AsyncDownloadUrlClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncStorageClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
