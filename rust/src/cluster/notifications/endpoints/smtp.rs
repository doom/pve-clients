pub mod name;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Author of the mail. Defaults to 'Proxmox VE'."]
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
    #[serde(rename = "from-address")]
    pub from_address: String,
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
    #[doc = "Determine which encryption method shall be used for the connection."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mode: Option<String>,
    #[doc = "The name of the endpoint."]
    pub name: String,
    #[doc = "Show if this entry was created by a user or was built-in"]
    pub origin: String,
    #[doc = "The port to be used. Defaults to 465 for TLS based connections, 587 for STARTTLS based connections and port 25 for insecure plain-text connections."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub port: Option<u64>,
    #[doc = "The address of the SMTP server."]
    pub server: String,
    #[doc = "Username for SMTP authentication"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Author of the mail. Defaults to 'Proxmox VE'."]
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
    #[serde(rename = "from-address")]
    pub from_address: String,
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
    #[doc = "Determine which encryption method shall be used for the connection."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mode: Option<String>,
    #[doc = "The name of the endpoint."]
    pub name: String,
    #[doc = "Password for SMTP authentication"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub password: Option<String>,
    #[doc = "The port to be used. Defaults to 465 for TLS based connections, 587 for STARTTLS based connections and port 25 for insecure plain-text connections."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub port: Option<u64>,
    #[doc = "The address of the SMTP server."]
    pub server: String,
    #[doc = "Username for SMTP authentication"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub username: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SmtpClient<T> {
    client: T,
    path: String,
}

impl<T> SmtpClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "smtp"),
        }
    }

    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> SmtpClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Returns a list of all smtp endpoints"]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create a new smtp endpoint"]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSmtpClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSmtpClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "smtp"),
        }
    }

    pub fn name(&self, name: &str) -> name::AsyncNameClient<T> {
        name::AsyncNameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> AsyncSmtpClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Returns a list of all smtp endpoints"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create a new smtp endpoint"]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
