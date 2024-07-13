#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "OpenId authorization code."]
    pub code: String,
    #[doc = "Redirection Url. The client should set this to the used server url (location.origin)."]
    #[serde(rename = "redirect-url")]
    pub redirect_url: String,
    #[doc = "OpenId state."]
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {
    #[serde(rename = "CSRFPreventionToken")]
    pub csrfprevention_token: String,
    pub cap: Cap,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub clustername: Option<String>,
    pub ticket: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Cap {}

#[derive(Debug, Clone)]
pub struct LoginClient<T> {
    client: T,
    path: String,
}

impl<T> LoginClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "login"),
        }
    }
}
impl<T> LoginClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = " Verify OpenID authorization code and create a ticket."]
    pub fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncLoginClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncLoginClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "login"),
        }
    }
}
impl<T> AsyncLoginClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = " Verify OpenID authorization code and create a ticket."]
    pub async fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
