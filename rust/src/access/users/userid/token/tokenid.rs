#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
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
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
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
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {
    #[doc = "The full token id."]
    #[serde(rename = "full-tokenid")]
    pub full_tokenid: String,
    pub info: Info,
    #[doc = "API token value used for authentication."]
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Info {
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
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
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
}

#[doc = "Updated token information."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutResponseItem {
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
}

#[derive(Debug, Clone)]
pub struct TokenidClient<T> {
    client: T,
    path: String,
}

impl<T> TokenidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, tokenid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, tokenid),
        }
    }
}
impl<T> TokenidClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Remove API token for a specific user."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Get specific API token information."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Generate a new API token for a specific user. NOTE: returns API token value, which needs to be stored as it cannot be retrieved afterwards!"]
    pub fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters)
    }

    #[doc = "Update API token for a specific user."]
    pub fn put(&self, parameters: PutParameters) -> Result<PutResponseItem, T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncTokenidClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncTokenidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, tokenid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, tokenid),
        }
    }
}
impl<T> AsyncTokenidClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Remove API token for a specific user."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Get specific API token information."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Generate a new API token for a specific user. NOTE: returns API token value, which needs to be stored as it cannot be retrieved afterwards!"]
    pub async fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters).await
    }

    #[doc = "Update API token for a specific user."]
    pub async fn put(&self, parameters: PutParameters) -> Result<PutResponseItem, T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
