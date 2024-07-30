pub mod realm;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "A comment. The GUI use this text when you select a domain (Realm) on the login window."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    pub realm: String,
    #[doc = "Two-factor authentication provider."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tfa: Option<String>,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Specifies the Authentication Context Class Reference values that theAuthorization Server is being requested to use for the Auth Request."]
    #[serde(
        rename = "acr-values",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub acr_values: Option<String>,
    #[doc = "Automatically create users if they do not exist."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub autocreate: Option<bool>,
    #[doc = "LDAP base domain name"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub base_dn: Option<String>,
    #[doc = "LDAP bind domain name"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bind_dn: Option<String>,
    #[doc = "Path to the CA certificate store"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub capath: Option<String>,
    #[doc = "username is case-sensitive"]
    #[serde(
        rename = "case-sensitive",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub case_sensitive: Option<bool>,
    #[doc = "Path to the client certificate"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cert: Option<String>,
    #[doc = "Path to the client certificate key"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub certkey: Option<String>,
    #[doc = "Check bind connection to the server."]
    #[serde(
        rename = "check-connection",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub check_connection: Option<bool>,
    #[doc = "OpenID Client ID"]
    #[serde(rename = "client-id", skip_serializing_if = "Option::is_none", default)]
    pub client_id: Option<String>,
    #[doc = "OpenID Client Key"]
    #[serde(
        rename = "client-key",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub client_key: Option<String>,
    #[doc = "Description."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Use this as default realm"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub default: Option<bool>,
    #[doc = "AD domain name"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub domain: Option<String>,
    #[doc = "LDAP filter for user sync."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub filter: Option<String>,
    #[doc = "The objectclasses for groups."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub group_classes: Option<String>,
    #[doc = "LDAP base domain name for group sync. If not set, the base_dn will be used."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub group_dn: Option<String>,
    #[doc = "LDAP filter for group sync."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub group_filter: Option<String>,
    #[doc = "LDAP attribute representing a groups name. If not set or found, the first value of the DN will be used as name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub group_name_attr: Option<String>,
    #[doc = "OpenID Issuer Url"]
    #[serde(
        rename = "issuer-url",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub issuer_url: Option<String>,
    #[doc = "LDAP protocol mode."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mode: Option<String>,
    #[doc = "LDAP bind password. Will be stored in '/etc/pve/priv/realm/<REALM>.pw'."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub password: Option<String>,
    #[doc = "Server port."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub port: Option<u64>,
    #[doc = "Specifies whether the Authorization Server prompts the End-User for reauthentication and consent."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub prompt: Option<String>,
    #[doc = "Authentication domain ID"]
    pub realm: String,
    #[doc = "Specifies the scopes (user details) that should be authorized and returned, for example 'email' or 'profile'."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub scopes: Option<String>,
    #[doc = "Use secure LDAPS protocol. DEPRECATED: use 'mode' instead."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub secure: Option<bool>,
    #[doc = "Server IP address (or DNS name)"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub server1: Option<String>,
    #[doc = "Fallback Server IP address (or DNS name)"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub server2: Option<String>,
    #[doc = "LDAPS TLS/SSL version. It's not recommended to use version older than 1.2!"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sslversion: Option<String>,
    #[doc = "The default options for behavior of synchronizations."]
    #[serde(
        rename = "sync-defaults-options",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sync_defaults_options: Option<String>,
    #[doc = "Comma separated list of key=value pairs for specifying which LDAP attributes map to which PVE user field. For example, to map the LDAP attribute 'mail' to PVEs 'email', write  'email=mail'. By default, each PVE user field is represented  by an LDAP attribute of the same name."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sync_attributes: Option<String>,
    #[doc = "Use Two-factor authentication."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tfa: Option<String>,
    #[doc = "Realm type."]
    pub r#type: String,
    #[doc = "LDAP user attribute name"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub user_attr: Option<String>,
    #[doc = "The objectclasses for users."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub user_classes: Option<String>,
    #[doc = "OpenID claim used to generate the unique username."]
    #[serde(
        rename = "username-claim",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub username_claim: Option<String>,
    #[doc = "Verify the server's SSL certificate"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub verify: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct DomainsClient<T> {
    client: T,
    path: String,
}

impl<T> DomainsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "domains"),
        }
    }

    pub fn realm(&self, realm: &str) -> realm::RealmClient<T> {
        realm::RealmClient::<T>::new(self.client.clone(), &self.path, realm)
    }
}
impl<T> DomainsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Authentication domain index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Add an authentication server."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncDomainsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncDomainsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "domains"),
        }
    }

    pub fn realm(&self, realm: &str) -> realm::AsyncRealmClient<T> {
        realm::AsyncRealmClient::<T>::new(self.client.clone(), &self.path, realm)
    }
}
impl<T> AsyncDomainsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Authentication domain index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Add an authentication server."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
