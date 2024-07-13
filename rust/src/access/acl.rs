#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Access control path"]
    pub path: String,
    #[doc = "Allow to propagate (inherit) permissions."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub propagate: Option<bool>,
    pub roleid: String,
    pub r#type: String,
    pub ugid: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "Remove permissions (instead of adding it)."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub delete: Option<bool>,
    #[doc = "List of groups."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub groups: Option<String>,
    #[doc = "Access control path"]
    pub path: String,
    #[doc = "Allow to propagate (inherit) permissions."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub propagate: Option<bool>,
    #[doc = "List of roles."]
    pub roles: String,
    #[doc = "List of API tokens."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tokens: Option<String>,
    #[doc = "List of users."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub users: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AclClient<T> {
    client: T,
    path: String,
}

impl<T> AclClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "acl"),
        }
    }
}
impl<T> AclClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get Access Control List (ACLs)."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update Access Control List (add or remove permissions)."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncAclClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncAclClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "acl"),
        }
    }
}
impl<T> AsyncAclClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get Access Control List (ACLs)."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update Access Control List (add or remove permissions)."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
