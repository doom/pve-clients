#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "URL of ACME CA directory endpoint."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub directory: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TosClient<T> {
    client: T,
    path: String,
}

impl<T> TosClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "tos"),
        }
    }
}
impl<T> TosClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Retrieve ACME TermsOfService URL from CA."]
    pub fn get(&self, parameters: GetParameters) -> Result<Option<String>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncTosClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncTosClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "tos"),
        }
    }
}
impl<T> AsyncTosClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Retrieve ACME TermsOfService URL from CA."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Option<String>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
