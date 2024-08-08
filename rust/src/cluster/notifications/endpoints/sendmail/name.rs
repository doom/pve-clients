#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Author of the mail"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub author: Option<String>,
    #[doc = "Comment"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Disable this target"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub disable: Option<bool>,
    #[doc = "`From` address for the mail"]
    #[serde(
        rename = "from-address",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub from_address: Option<String>,
    #[doc = "List of email recipients"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mailto: Option<Vec<String>>,
    #[doc = "List of users"]
    #[serde(
        rename = "mailto-user",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub mailto_user: Option<Vec<String>>,
    #[doc = "The name of the endpoint."]
    pub name: String,
}

impl GetResponseItem {
    pub fn new(name: String) -> Self {
        Self {
            name,
            author: Default::default(),
            comment: Default::default(),
            digest: Default::default(),
            disable: Default::default(),
            from_address: Default::default(),
            mailto: Default::default(),
            mailto_user: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "Author of the mail"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub author: Option<String>,
    #[doc = "Comment"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<Vec<String>>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Disable this target"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub disable: Option<bool>,
    #[doc = "`From` address for the mail"]
    #[serde(
        rename = "from-address",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub from_address: Option<String>,
    #[doc = "List of email recipients"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mailto: Option<Vec<String>>,
    #[doc = "List of users"]
    #[serde(
        rename = "mailto-user",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub mailto_user: Option<Vec<String>>,
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
    #[doc = "Remove sendmail endpoint"]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Return a specific sendmail endpoint"]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update existing sendmail endpoint"]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
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
    #[doc = "Remove sendmail endpoint"]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Return a specific sendmail endpoint"]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update existing sendmail endpoint"]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
