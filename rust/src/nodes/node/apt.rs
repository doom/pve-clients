pub mod changelog;
pub mod repositories;
pub mod update;
pub mod versions;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct AptClient<T> {
    client: T,
    path: String,
}

impl<T> AptClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "apt"),
        }
    }

    pub fn update(&self) -> update::UpdateClient<T> {
        update::UpdateClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn changelog(&self) -> changelog::ChangelogClient<T> {
        changelog::ChangelogClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn repositories(&self) -> repositories::RepositoriesClient<T> {
        repositories::RepositoriesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn versions(&self) -> versions::VersionsClient<T> {
        versions::VersionsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AptClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Directory index for apt (Advanced Package Tool)."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncAptClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncAptClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "apt"),
        }
    }

    pub fn update(&self) -> update::AsyncUpdateClient<T> {
        update::AsyncUpdateClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn changelog(&self) -> changelog::AsyncChangelogClient<T> {
        changelog::AsyncChangelogClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn repositories(&self) -> repositories::AsyncRepositoriesClient<T> {
        repositories::AsyncRepositoriesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn versions(&self) -> versions::AsyncVersionsClient<T> {
        versions::AsyncVersionsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncAptClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Directory index for apt (Advanced Package Tool)."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
