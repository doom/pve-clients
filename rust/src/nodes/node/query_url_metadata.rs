#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "The URL to query the metadata from."]
    pub url: String,
    #[doc = "If false, no SSL/TLS certificates will be verified."]
    #[serde(
        rename = "verify-certificates",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub verify_certificates: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mimetype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub size: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct QueryUrlMetadataClient<T> {
    client: T,
    path: String,
}

impl<T> QueryUrlMetadataClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "query-url-metadata"),
        }
    }
}
impl<T> QueryUrlMetadataClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Query metadata of an URL: file size, file name and mime type."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncQueryUrlMetadataClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncQueryUrlMetadataClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "query-url-metadata"),
        }
    }
}
impl<T> AsyncQueryUrlMetadataClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Query metadata of an URL: file size, file name and mime type."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
