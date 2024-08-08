#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "base64-path to the directory or file being listed, or \"/\"."]
    pub filepath: String,
    #[doc = "Backup volume ID or name. Currently only PBS snapshots are supported."]
    pub volume: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "base64 path of the current entry"]
    pub filepath: String,
    #[doc = "If this entry is a leaf in the directory graph."]
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub leaf: bool,
    #[doc = "Entry last-modified time (unix timestamp)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mtime: Option<u64>,
    #[doc = "Entry file size."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub size: Option<u64>,
    #[doc = "Entry display text."]
    pub text: String,
    #[doc = "Entry type."]
    pub r#type: String,
}

impl GetResponseItem {
    pub fn new(filepath: String, leaf: bool, text: String, r#type: String) -> Self {
        Self {
            filepath,
            leaf,
            text,
            r#type,
            mtime: Default::default(),
            size: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListClient<T> {
    client: T,
    path: String,
}

impl<T> ListClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "list"),
        }
    }
}
impl<T> ListClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List files and directories for single file restore under the given path."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncListClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncListClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "list"),
        }
    }
}
impl<T> AsyncListClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List files and directories for single file restore under the given path."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
