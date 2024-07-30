pub mod name;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Contact email addresses."]
    pub contact: String,
    #[doc = "URL of ACME CA directory endpoint."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub directory: Option<String>,
    #[doc = "HMAC key for External Account Binding."]
    #[serde(
        rename = "eab-hmac-key",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub eab_hmac_key: Option<String>,
    #[doc = "Key Identifier for External Account Binding."]
    #[serde(rename = "eab-kid", skip_serializing_if = "Option::is_none", default)]
    pub eab_kid: Option<String>,
    #[doc = "ACME account config file name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    #[doc = "URL of CA TermsOfService - setting this indicates agreement."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tos_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AccountClient<T> {
    client: T,
    path: String,
}

impl<T> AccountClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "account"),
        }
    }

    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> AccountClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "ACMEAccount index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Register a new ACME account with CA."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncAccountClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncAccountClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "account"),
        }
    }

    pub fn name(&self, name: &str) -> name::AsyncNameClient<T> {
        name::AsyncNameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> AsyncAccountClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "ACMEAccount index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Register a new ACME account with CA."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
