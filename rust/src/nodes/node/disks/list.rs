#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "Also include partitions."]
    #[serde(
        rename = "include-partitions",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub include_partitions: Option<bool>,
    #[doc = "Skip smart checks."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub skipsmart: Option<bool>,
    #[doc = "Only list specific types of disks."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The device path"]
    pub devpath: String,
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub gpt: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub health: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub model: Option<String>,
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub mounted: bool,
    pub osdid: u64,
    #[serde(rename = "osdid-list")]
    pub osdid_list: Vec<u64>,
    #[doc = "For partitions only. The device path of the disk the partition resides on."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub serial: Option<String>,
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub used: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub wwn: Option<String>,
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
    #[doc = "List local disks."]
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
    #[doc = "List local disks."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
