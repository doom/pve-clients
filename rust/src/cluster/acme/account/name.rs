#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub account: Option<Account>,
    #[doc = "URL of ACME CA directory endpoint."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tos: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Account {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "Contact email addresses."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub contact: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NameClient<T> {
    client: T,
    path: String,
}

impl<T> NameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
impl<T> NameClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Deactivate existing ACME account at CA."]
    pub fn delete(&self) -> Result<String, T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Return existing ACME account information."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update existing ACME account information with CA. Note: not specifying any new account information triggers a refresh."]
    pub fn put(&self, parameters: PutParameters) -> Result<String, T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncNameClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
impl<T> AsyncNameClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Deactivate existing ACME account at CA."]
    pub async fn delete(&self) -> Result<String, T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Return existing ACME account information."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update existing ACME account information with CA. Note: not specifying any new account information triggers a refresh."]
    pub async fn put(&self, parameters: PutParameters) -> Result<String, T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
