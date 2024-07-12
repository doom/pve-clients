pub mod reload;
pub mod restart;
pub mod start;
pub mod state;
pub mod stop;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub subdir: String,
}

#[derive(Debug, Clone)]
pub struct ServiceClient<T> {
    client: T,
    path: String,
}

impl<T> ServiceClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, service: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, service),
        }
    }

    pub fn state(&self) -> state::StateClient<T> {
        state::StateClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn start(&self) -> start::StartClient<T> {
        start::StartClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn stop(&self) -> stop::StopClient<T> {
        stop::StopClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn restart(&self) -> restart::RestartClient<T> {
        restart::RestartClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn reload(&self) -> reload::ReloadClient<T> {
        reload::ReloadClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ServiceClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Directory index"]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncServiceClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncServiceClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, service: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, service),
        }
    }

    pub fn state(&self) -> state::AsyncStateClient<T> {
        state::AsyncStateClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn start(&self) -> start::AsyncStartClient<T> {
        start::AsyncStartClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn stop(&self) -> stop::AsyncStopClient<T> {
        stop::AsyncStopClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn restart(&self) -> restart::AsyncRestartClient<T> {
        restart::AsyncRestartClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn reload(&self) -> reload::AsyncReloadClient<T> {
        reload::AsyncReloadClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncServiceClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Directory index"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
