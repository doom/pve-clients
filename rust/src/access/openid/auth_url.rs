#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Authentication domain ID"]
    pub realm: String,
    #[doc = "Redirection Url. The client should set this to the used server url (location.origin)."]
    #[serde(rename = "redirect-url")]
    pub redirect_url: String,
}

#[derive(Debug, Clone)]
pub struct AuthUrlClient<T> {
    client: T,
    path: String,
}

impl<T> AuthUrlClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "auth-url"),
        }
    }
}
impl<T> AuthUrlClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get the OpenId Authorization Url for the specified realm."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncAuthUrlClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncAuthUrlClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "auth-url"),
        }
    }
}
impl<T> AsyncAuthUrlClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get the OpenId Authorization Url for the specified realm."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
