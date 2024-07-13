pub mod groups;
pub mod resources;
pub mod status;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct HaClient<T> {
    client: T,
    path: String,
}

impl<T> HaClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "ha"),
        }
    }

    pub fn resources(&self) -> resources::ResourcesClient<T> {
        resources::ResourcesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn groups(&self) -> groups::GroupsClient<T> {
        groups::GroupsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> HaClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Directory index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncHaClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncHaClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "ha"),
        }
    }

    pub fn resources(&self) -> resources::AsyncResourcesClient<T> {
        resources::AsyncResourcesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn groups(&self) -> groups::AsyncGroupsClient<T> {
        groups::AsyncGroupsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn status(&self) -> status::AsyncStatusClient<T> {
        status::AsyncStatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncHaClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Directory index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
