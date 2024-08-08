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
    #[doc = "Content type."]
    pub content: String,
    #[doc = "The name of the file to create. Caution: This will be normalized!"]
    pub filename: String,
    #[doc = "The source file name. This parameter is usually set by the REST handler. You can only overwrite it when connecting to the trusted port on localhost."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tmpfilename: Option<String>,
}

impl PostParameters {
    pub fn new(content: String, filename: String) -> Self {
        Self {
            content,
            filename,
            checksum: Default::default(),
            checksum_algorithm: Default::default(),
            tmpfilename: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UploadClient<T> {
    client: T,
    path: String,
}

impl<T> UploadClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "upload"),
        }
    }
}
impl<T> UploadClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Upload templates and ISO images."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncUploadClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncUploadClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "upload"),
        }
    }
}
impl<T> AsyncUploadClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Upload templates and ISO images."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
