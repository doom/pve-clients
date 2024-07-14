#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct NetworkGetInterfacesClient<T> {
    client: T,
    path: String,
}

impl<T> NetworkGetInterfacesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "network-get-interfaces"),
        }
    }
}
impl<T> NetworkGetInterfacesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Execute network-get-interfaces."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncNetworkGetInterfacesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNetworkGetInterfacesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "network-get-interfaces"),
        }
    }
}
impl<T> AsyncNetworkGetInterfacesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Execute network-get-interfaces."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
