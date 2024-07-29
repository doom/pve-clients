pub mod log;
pub mod status;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct UpidClient<T> {
    client: T,
    path: String,
}

impl<T> UpidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, upid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, upid),
        }
    }

    pub fn log(&self) -> log::LogClient<T> {
        log::LogClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> UpidClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Stop a task."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncUpidClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncUpidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, upid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, upid),
        }
    }

    pub fn log(&self) -> log::AsyncLogClient<T> {
        log::AsyncLogClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn status(&self) -> status::AsyncStatusClient<T> {
        status::AsyncStatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncUpidClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Stop a task."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
