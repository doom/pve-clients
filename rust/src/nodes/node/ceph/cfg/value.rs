#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "List of <section>:<config key> items."]
    #[serde(rename = "config-keys")]
    pub config_keys: String,
}

#[doc = "Contains {section}->{key} children with the values"]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct ValueClient<T> {
    client: T,
    path: String,
}

impl<T> ValueClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "value"),
        }
    }
}
impl<T> ValueClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get configured values from either the config file or config DB."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncValueClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncValueClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "value"),
        }
    }
}
impl<T> AsyncValueClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get configured values from either the config file or config DB."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
