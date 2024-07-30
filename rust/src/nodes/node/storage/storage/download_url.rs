#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "The expected checksum of the file."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub checksum: Option<String>,
    #[doc = "The algorithm to calculate the checksum of the file."]
    #[serde(
        rename = "checksum-algorithm",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub checksum_algorithm: Option<String>,
    #[doc = "Decompress the downloaded file using the specified compression algorithm."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub compression: Option<String>,
    #[doc = "Content type."]
    pub content: String,
    #[doc = "The name of the file to create. Caution: This will be normalized!"]
    pub filename: String,
    #[doc = "The URL to download the file from."]
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

#[derive(Debug, Clone)]
pub struct DownloadUrlClient<T> {
    client: T,
    path: String,
}

impl<T> DownloadUrlClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "download-url"),
        }
    }
}
impl<T> DownloadUrlClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Download templates and ISO images by using an URL."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncDownloadUrlClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncDownloadUrlClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "download-url"),
        }
    }
}
impl<T> AsyncDownloadUrlClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Download templates and ISO images by using an URL."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
