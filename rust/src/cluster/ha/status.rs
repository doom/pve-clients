pub mod current;
pub mod manager_status;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct StatusClient<T> {
    client: T,
    path: String,
}

impl<T> StatusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "status"),
        }
    }

    pub fn current(&self) -> current::CurrentClient<T> {
        current::CurrentClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn manager_status(&self) -> manager_status::ManagerStatusClient<T> {
        manager_status::ManagerStatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Directory index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncStatusClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncStatusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "status"),
        }
    }

    pub fn current(&self) -> current::AsyncCurrentClient<T> {
        current::AsyncCurrentClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn manager_status(&self) -> manager_status::AsyncManagerStatusClient<T> {
        manager_status::AsyncManagerStatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncStatusClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Directory index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
