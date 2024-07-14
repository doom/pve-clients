pub mod tfa;
pub mod token;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub email: Option<String>,
    #[doc = "Enable the account (default). You can set this to '0' to disable the account"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enable: Option<bool>,
    #[doc = "Account expiration date (seconds since epoch). '0' means no expiration date."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub expire: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub groups: Option<Vec<String>>,
    #[doc = "Keys for two factor auth (yubico)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub keys: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lastname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tokens: Option<Tokens>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Tokens {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub append: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub email: Option<String>,
    #[doc = "Enable the account (default). You can set this to '0' to disable the account"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enable: Option<bool>,
    #[doc = "Account expiration date (seconds since epoch). '0' means no expiration date."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub expire: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub groups: Option<String>,
    #[doc = "Keys for two factor auth (yubico)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub keys: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lastname: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UseridClient<T> {
    client: T,
    path: String,
}

impl<T> UseridClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, userid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, userid),
        }
    }

    pub fn tfa(&self) -> tfa::TfaClient<T> {
        tfa::TfaClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn token(&self) -> token::TokenClient<T> {
        token::TokenClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> UseridClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete user."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Get user configuration."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update user configuration."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncUseridClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncUseridClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, userid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, userid),
        }
    }

    pub fn tfa(&self) -> tfa::AsyncTfaClient<T> {
        tfa::AsyncTfaClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn token(&self) -> token::AsyncTokenClient<T> {
        token::AsyncTokenClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncUseridClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete user."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Get user configuration."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update user configuration."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
