pub mod userid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "Optional filter for enable property."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enabled: Option<bool>,
    #[doc = "Include group and token information."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub full: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
    pub groups: Option<String>,
    #[doc = "Keys for two factor auth (yubico)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub keys: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lastname: Option<String>,
    #[doc = "The type of the users realm"]
    #[serde(
        rename = "realm-type",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub realm_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tokens: Option<Vec<GetResponseItemTokensItem>>,
    #[doc = "Full User ID, in the `name@realm` format."]
    pub userid: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItemTokensItem {
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
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
    #[doc = "Initial password."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub password: Option<String>,
    #[doc = "Full User ID, in the `name@realm` format."]
    pub userid: String,
}

#[derive(Debug, Clone)]
pub struct UsersClient<T> {
    client: T,
    path: String,
}

impl<T> UsersClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "users"),
        }
    }

    pub fn userid(&self, userid: &str) -> userid::UseridClient<T> {
        userid::UseridClient::<T>::new(self.client.clone(), &self.path, userid)
    }
}
impl<T> UsersClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "User index."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Create new user."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncUsersClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncUsersClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "users"),
        }
    }

    pub fn userid(&self, userid: &str) -> userid::AsyncUseridClient<T> {
        userid::AsyncUseridClient::<T>::new(self.client.clone(), &self.path, userid)
    }
}
impl<T> AsyncUsersClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "User index."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Create new user."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
