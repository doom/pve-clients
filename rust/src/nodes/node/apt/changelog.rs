#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "Package name."]
    pub name: String,
    #[doc = "Package version."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub version: Option<String>,
}

impl GetParameters {
    pub fn new(name: String) -> Self {
        Self {
            name,
            version: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChangelogClient<T> {
    client: T,
    path: String,
}

impl<T> ChangelogClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "changelog"),
        }
    }
}
impl<T> ChangelogClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get package changelogs."]
    pub fn get(&self, parameters: GetParameters) -> Result<String, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncChangelogClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncChangelogClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "changelog"),
        }
    }
}
impl<T> AsyncChangelogClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get package changelogs."]
    pub async fn get(&self, parameters: GetParameters) -> Result<String, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
