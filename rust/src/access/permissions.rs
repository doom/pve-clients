#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "Only dump this specific path, not the whole tree."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub path: Option<String>,
    #[doc = "User ID or full API token ID"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub userid: Option<String>,
}

#[doc = "Map of \"path\" => (Map of \"privilege\" => \"propagate boolean\")."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct PermissionsClient<T> {
    client: T,
    path: String,
}

impl<T> PermissionsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "permissions"),
        }
    }
}
impl<T> PermissionsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Retrieve effective permissions of given user/token."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncPermissionsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncPermissionsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "permissions"),
        }
    }
}
impl<T> AsyncPermissionsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Retrieve effective permissions of given user/token."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
