#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "Force physical removal. Without this, we simple remove the disk from the config file and create an additional configuration entry called 'unused[n]', which contains the volume ID. Unlink of unused[n] always cause physical removal."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub force: Option<bool>,
    #[doc = "A list of disk IDs you want to delete."]
    pub idlist: String,
}

#[derive(Debug, Clone)]
pub struct UnlinkClient<T> {
    client: T,
    path: String,
}

impl<T> UnlinkClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "unlink"),
        }
    }
}
impl<T> UnlinkClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Unlink/delete disk images."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncUnlinkClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncUnlinkClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "unlink"),
        }
    }
}
impl<T> AsyncUnlinkClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Unlink/delete disk images."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
