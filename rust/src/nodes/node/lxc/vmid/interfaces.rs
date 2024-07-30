#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The MAC address of the interface"]
    pub hwaddr: String,
    #[doc = "The IPv4 address of the interface"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub inet: Option<String>,
    #[doc = "The IPv6 address of the interface"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub inet6: Option<String>,
    #[doc = "The name of the interface"]
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct InterfacesClient<T> {
    client: T,
    path: String,
}

impl<T> InterfacesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "interfaces"),
        }
    }
}
impl<T> InterfacesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get IP addresses of the specified container interface."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncInterfacesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncInterfacesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "interfaces"),
        }
    }
}
impl<T> AsyncInterfacesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get IP addresses of the specified container interface."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
