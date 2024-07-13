pub mod tokenid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "API token expiration date (seconds since epoch). '0' means no expiration date."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub expire: Option<u64>,
    #[doc = "Restrict API token privileges with separate ACLs (default), or give full privileges of corresponding user."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub privsep: Option<bool>,
    #[doc = "User-specific token identifier."]
    pub tokenid: String,
}

#[derive(Debug, Clone)]
pub struct TokenClient<T> {
    client: T,
    path: String,
}

impl<T> TokenClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "token"),
        }
    }

    pub fn tokenid(&self, tokenid: &str) -> tokenid::TokenidClient<T> {
        tokenid::TokenidClient::<T>::new(self.client.clone(), &self.path, tokenid)
    }
}
impl<T> TokenClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get user API tokens."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncTokenClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncTokenClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "token"),
        }
    }

    pub fn tokenid(&self, tokenid: &str) -> tokenid::AsyncTokenidClient<T> {
        tokenid::AsyncTokenidClient::<T>::new(self.client.clone(), &self.path, tokenid)
    }
}
impl<T> AsyncTokenClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get user API tokens."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
