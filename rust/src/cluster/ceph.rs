pub mod flags;
pub mod metadata;
pub mod status;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct CephClient<T> {
    client: T,
    path: String,
}

impl<T> CephClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "ceph"),
        }
    }

    pub fn metadata(&self) -> metadata::MetadataClient<T> {
        metadata::MetadataClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn flags(&self) -> flags::FlagsClient<T> {
        flags::FlagsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Cluster ceph index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCephClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCephClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "ceph"),
        }
    }

    pub fn metadata(&self) -> metadata::AsyncMetadataClient<T> {
        metadata::AsyncMetadataClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn status(&self) -> status::AsyncStatusClient<T> {
        status::AsyncStatusClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn flags(&self) -> flags::AsyncFlagsClient<T> {
        flags::AsyncFlagsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncCephClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Cluster ceph index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
