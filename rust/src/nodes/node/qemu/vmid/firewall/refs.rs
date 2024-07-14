#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "Only list references of specified type."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Clone)]
pub struct RefsClient<T> {
    client: T,
    path: String,
}

impl<T> RefsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "refs"),
        }
    }
}
impl<T> RefsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Lists possible IPSet/Alias reference which are allowed in source/dest properties."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncRefsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncRefsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "refs"),
        }
    }
}
impl<T> AsyncRefsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Lists possible IPSet/Alias reference which are allowed in source/dest properties."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
