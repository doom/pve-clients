#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Author of the mail. Defaults to 'Proxmox VE'."]
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
    #[doc = "The port to be used. Defaults to 465 for TLS based connections, 587 for STARTTLS based connections and port 25 for insecure plain-text connections."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub port: Option<u64>,
    #[doc = "The address of the SMTP server."]
    pub server: String,
    #[doc = "Username for SMTP authentication"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "Author of the mail. Defaults to 'Proxmox VE'."]
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
    #[doc = "Determine which encryption method shall be used for the connection."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mode: Option<String>,
    #[doc = "Password for SMTP authentication"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub password: Option<String>,
    #[doc = "The port to be used. Defaults to 465 for TLS based connections, 587 for STARTTLS based connections and port 25 for insecure plain-text connections."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub port: Option<u64>,
    #[doc = "The address of the SMTP server."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub server: Option<String>,
    #[doc = "Username for SMTP authentication"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub username: Option<String>,
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
    #[doc = "Remove smtp endpoint"]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Return a specific smtp endpoint"]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update existing smtp endpoint"]
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
    #[doc = "Remove smtp endpoint"]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Return a specific smtp endpoint"]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update existing smtp endpoint"]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
