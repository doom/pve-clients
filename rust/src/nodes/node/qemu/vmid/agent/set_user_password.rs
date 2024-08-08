#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "set to 1 if the password has already been passed through crypt()"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub crypted: Option<bool>,
    #[doc = "The new password."]
    pub password: String,
    #[doc = "The user to set the password for."]
    pub username: String,
}

impl PostParameters {
    pub fn new(password: String, username: String) -> Self {
        Self {
            password,
            username,
            crypted: Default::default(),
        }
    }
}

#[doc = "Returns an object with a single `result` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostResponseItem {}

#[derive(Debug, Clone)]
pub struct SetUserPasswordClient<T> {
    client: T,
    path: String,
}

impl<T> SetUserPasswordClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "set-user-password"),
        }
    }
}
impl<T> SetUserPasswordClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Sets the password for the given user to the given password"]
    pub fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSetUserPasswordClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSetUserPasswordClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "set-user-password"),
        }
    }
}
impl<T> AsyncSetUserPasswordClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Sets the password for the given user to the given password"]
    pub async fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
