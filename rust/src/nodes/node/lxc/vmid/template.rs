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
    pub fn post(&self) -> Result<(), T::Error> {
        self.client.post(&self.path, &())
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
    pub async fn post(&self) -> Result<(), T::Error> {
        self.client.post(&self.path, &()).await
    }
}
