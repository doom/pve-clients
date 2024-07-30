#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "The current password of the user performing the change."]
    #[serde(
        rename = "confirmation-password",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub confirmation_password: Option<String>,
    #[doc = "The new password."]
    pub password: String,
    #[doc = "Full User ID, in the `name@realm` format."]
    pub userid: String,
}

#[derive(Debug, Clone)]
pub struct PasswordClient<T> {
    client: T,
    path: String,
}

impl<T> PasswordClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "password"),
        }
    }
}
impl<T> PasswordClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Change user password."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncPasswordClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncPasswordClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "password"),
        }
    }
}
impl<T> AsyncPasswordClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Change user password."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
