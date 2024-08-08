#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Indicates a pending delete request if present and not 0. The value 2 indicates a force-delete request."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<u64>,
    #[doc = "Configuration option name."]
    pub key: String,
    #[doc = "Pending value."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pending: Option<String>,
    #[doc = "Current value."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub value: Option<String>,
}

impl GetResponseItem {
    pub fn new(key: String) -> Self {
        Self {
            key,
            delete: Default::default(),
            pending: Default::default(),
            value: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PendingClient<T> {
    client: T,
    path: String,
}

impl<T> PendingClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "pending"),
        }
    }
}
impl<T> PendingClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get the virtual machine configuration with both current and pending values."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncPendingClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncPendingClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "pending"),
        }
    }
}
impl<T> AsyncPendingClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get the virtual machine configuration with both current and pending values."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
