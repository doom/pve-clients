#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "The disk you want to resize."]
    pub disk: String,
    #[doc = "The new size. With the `+` sign the value is added to the actual size of the volume and without it, the value is taken as an absolute one. Shrinking disk size is not supported."]
    pub size: String,
    #[doc = "Ignore locks - only root is allowed to use this option."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub skiplock: Option<bool>,
}

impl PutParameters {
    pub fn new(disk: String, size: String) -> Self {
        Self {
            disk,
            size,
            digest: Default::default(),
            skiplock: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResizeClient<T> {
    client: T,
    path: String,
}

impl<T> ResizeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "resize"),
        }
    }
}
impl<T> ResizeClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Extend volume size."]
    pub fn put(&self, parameters: PutParameters) -> Result<String, T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncResizeClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncResizeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "resize"),
        }
    }
}
impl<T> AsyncResizeClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Extend volume size."]
    pub async fn put(&self, parameters: PutParameters) -> Result<String, T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
