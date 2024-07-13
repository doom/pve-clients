#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "If you want to convert only 1 disk to base image."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub disk: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TemplateClient<T> {
    client: T,
    path: String,
}

impl<T> TemplateClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "template"),
        }
    }
}
impl<T> TemplateClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Create a Template."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncTemplateClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncTemplateClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "template"),
        }
    }
}
impl<T> AsyncTemplateClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Create a Template."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
