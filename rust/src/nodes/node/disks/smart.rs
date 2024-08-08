#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "Block device name"]
    pub disk: String,
    #[doc = "If true returns only the health status"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub healthonly: Option<bool>,
}

impl GetParameters {
    pub fn new(disk: String) -> Self {
        Self {
            disk,
            healthonly: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub attributes: Option<Vec<serde_json::Value>>,
    pub health: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

impl GetResponseItem {
    pub fn new(health: String) -> Self {
        Self {
            health,
            attributes: Default::default(),
            text: Default::default(),
            r#type: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SmartClient<T> {
    client: T,
    path: String,
}

impl<T> SmartClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "smart"),
        }
    }
}
impl<T> SmartClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get SMART Health of a disk."]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSmartClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSmartClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "smart"),
        }
    }
}
impl<T> AsyncSmartClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get SMART Health of a disk."]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
