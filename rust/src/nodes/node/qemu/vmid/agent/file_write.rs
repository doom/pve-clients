#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "The content to write into the file."]
    pub content: String,
    #[doc = "If set, the content will be encoded as base64 (required by QEMU).Otherwise the content needs to be encoded beforehand - defaults to true."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub encode: Option<bool>,
    #[doc = "The path to the file."]
    pub file: String,
}

impl PostParameters {
    pub fn new(content: String, file: String) -> Self {
        Self {
            content,
            file,
            encode: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileWriteClient<T> {
    client: T,
    path: String,
}

impl<T> FileWriteClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "file-write"),
        }
    }
}
impl<T> FileWriteClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Writes the given file via guest agent."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncFileWriteClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncFileWriteClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "file-write"),
        }
    }
}
impl<T> AsyncFileWriteClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Writes the given file via guest agent."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
