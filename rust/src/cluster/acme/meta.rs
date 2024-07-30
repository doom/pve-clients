#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "URL of ACME CA directory endpoint."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub directory: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {
    #[doc = "Hostnames referring to the ACME servers."]
    #[serde(
        rename = "caaIdentities",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub caa_identities: Option<Vec<String>>,
    #[doc = "EAB Required"]
    #[serde(
        rename = "externalAccountRequired",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub external_account_required: Option<bool>,
    #[doc = "ACME TermsOfService URL."]
    #[serde(
        rename = "termsOfService",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub terms_of_service: Option<String>,
    #[doc = "URL to more information about the ACME server."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub website: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MetaClient<T> {
    client: T,
    path: String,
}

impl<T> MetaClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "meta"),
        }
    }
}
impl<T> MetaClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Retrieve ACME Directory Meta Information"]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMetaClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMetaClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "meta"),
        }
    }
}
impl<T> AsyncMetaClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Retrieve ACME Directory Meta Information"]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
