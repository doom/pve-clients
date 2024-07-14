#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PostParameters {
    #[doc = "If set, instructs a deep scrub instead of a normal one."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub deep: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ScrubClient<T> {
    client: T,
    path: String,
}

impl<T> ScrubClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "scrub"),
        }
    }
}
impl<T> ScrubClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Instruct the OSD to scrub."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncScrubClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncScrubClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "scrub"),
        }
    }
}
impl<T> AsyncScrubClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Instruct the OSD to scrub."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
