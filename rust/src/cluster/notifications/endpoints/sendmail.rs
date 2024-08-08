pub mod name;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Author of the mail"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub author: Option<String>,
    #[doc = "Comment"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
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
    #[doc = "Show if this entry was created by a user or was built-in"]
    pub origin: String,
}

impl GetResponseItem {
    pub fn new(name: String, origin: String) -> Self {
        Self {
            name,
            origin,
            author: Default::default(),
            comment: Default::default(),
            disable: Default::default(),
            from_address: Default::default(),
            mailto: Default::default(),
            mailto_user: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Author of the mail"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub author: Option<String>,
    #[doc = "Comment"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
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

impl PostParameters {
    pub fn new(name: String) -> Self {
        Self {
            name,
            author: Default::default(),
            comment: Default::default(),
            disable: Default::default(),
            from_address: Default::default(),
            mailto: Default::default(),
            mailto_user: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SendmailClient<T> {
    client: T,
    path: String,
}

impl<T> SendmailClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "sendmail"),
        }
    }

    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> SendmailClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Returns a list of all sendmail endpoints"]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create a new sendmail endpoint"]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSendmailClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSendmailClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "sendmail"),
        }
    }

    pub fn name(&self, name: &str) -> name::AsyncNameClient<T> {
        name::AsyncNameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> AsyncSendmailClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Returns a list of all sendmail endpoints"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create a new sendmail endpoint"]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
