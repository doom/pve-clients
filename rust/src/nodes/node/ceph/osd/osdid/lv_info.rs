#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "OSD device type"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Creation time as reported by `lvs`."]
    pub creation_time: String,
    #[doc = "Name of the logical volume (LV)."]
    pub lv_name: String,
    #[doc = "Path to the logical volume (LV)."]
    pub lv_path: String,
    #[doc = "Size of the logical volume (LV)."]
    pub lv_size: u64,
    #[doc = "UUID of the logical volume (LV)."]
    pub lv_uuid: String,
    #[doc = "Name of the volume group (VG)."]
    pub vg_name: String,
}

#[derive(Debug, Clone)]
pub struct LvInfoClient<T> {
    client: T,
    path: String,
}

impl<T> LvInfoClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "lv-info"),
        }
    }
}
impl<T> LvInfoClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get OSD volume details"]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncLvInfoClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncLvInfoClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "lv-info"),
        }
    }
}
impl<T> AsyncLvInfoClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get OSD volume details"]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
