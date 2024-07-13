pub mod current;
pub mod reboot;
pub mod reset;
pub mod resume;
pub mod shutdown;
pub mod start;
pub mod stop;
pub mod suspend;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub subdir: String,
}

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

    pub fn start(&self) -> start::StartClient<T> {
        start::StartClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn stop(&self) -> stop::StopClient<T> {
        stop::StopClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn reset(&self) -> reset::ResetClient<T> {
        reset::ResetClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn shutdown(&self) -> shutdown::ShutdownClient<T> {
        shutdown::ShutdownClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn reboot(&self) -> reboot::RebootClient<T> {
        reboot::RebootClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn suspend(&self) -> suspend::SuspendClient<T> {
        suspend::SuspendClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn resume(&self) -> resume::ResumeClient<T> {
        resume::ResumeClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Directory index"]
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

    pub fn start(&self) -> start::AsyncStartClient<T> {
        start::AsyncStartClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn stop(&self) -> stop::AsyncStopClient<T> {
        stop::AsyncStopClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn reset(&self) -> reset::AsyncResetClient<T> {
        reset::AsyncResetClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn shutdown(&self) -> shutdown::AsyncShutdownClient<T> {
        shutdown::AsyncShutdownClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn reboot(&self) -> reboot::AsyncRebootClient<T> {
        reboot::AsyncRebootClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn suspend(&self) -> suspend::AsyncSuspendClient<T> {
        suspend::AsyncSuspendClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn resume(&self) -> resume::AsyncResumeClient<T> {
        resume::AsyncResumeClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncStatusClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Directory index"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
