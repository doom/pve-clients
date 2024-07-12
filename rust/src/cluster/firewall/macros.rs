#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "More verbose description (if available)."]
    pub descr: String,
    #[doc = "Macro name."]
    pub r#macro: String,
}

#[derive(Debug, Clone)]
pub struct MacrosClient<T> {
    client: T,
    path: String,
}

impl<T> MacrosClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "macros"),
        }
    }
}
impl<T> MacrosClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List available macros"]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMacrosClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMacrosClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "macros"),
        }
    }
}
impl<T> AsyncMacrosClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List available macros"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
