#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "The path to the file"]
    pub file: String,
}

#[doc = "Returns an object with a `content` property."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The content of the file, maximum 16777216"]
    pub content: String,
    #[doc = "If set to 1, the output is truncated and not complete"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct FileReadClient<T> {
    client: T,
    path: String,
}

impl<T> FileReadClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "file-read"),
        }
    }
}
impl<T> FileReadClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Reads the given file via guest agent. Is limited to 16777216 bytes."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncFileReadClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncFileReadClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "file-read"),
        }
    }
}
impl<T> AsyncFileReadClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Reads the given file via guest agent. Is limited to 16777216 bytes."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
