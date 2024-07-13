pub mod service;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct ServicesClient<T> {
    client: T,
    path: String,
}

impl<T> ServicesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "services"),
        }
    }

    pub fn service(&self, service: &str) -> service::ServiceClient<T> {
        service::ServiceClient::<T>::new(self.client.clone(), &self.path, service)
    }
}
impl<T> ServicesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Service list."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncServicesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncServicesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "services"),
        }
    }

    pub fn service(&self, service: &str) -> service::AsyncServiceClient<T> {
        service::AsyncServiceClient::<T>::new(self.client.clone(), &self.path, service)
    }
}
impl<T> AsyncServicesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Service list."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
