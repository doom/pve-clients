#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "Request all entries as an array."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub multiple: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The type of TFA the users realm has set, if any."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub realm: Option<String>,
    #[doc = "Array of the user configured TFA types, if any. Only available if 'multiple' was not passed."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub types: Option<Vec<String>>,
    #[doc = "The type of TFA the user has set, if any. Only set if 'multiple' was not passed."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub user: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TfaClient<T> {
    client: T,
    path: String,
}

impl<T> TfaClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "tfa"),
        }
    }
}
impl<T> TfaClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get user TFA types (Personal and Realm)."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncTfaClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncTfaClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "tfa"),
        }
    }
}
impl<T> AsyncTfaClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get user TFA types (Personal and Realm)."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
