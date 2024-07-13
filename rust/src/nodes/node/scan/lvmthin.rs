#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    pub vg: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The LVM Thin Pool name (LVM logical volume)."]
    pub lv: String,
}

#[derive(Debug, Clone)]
pub struct LvmthinClient<T> {
    client: T,
    path: String,
}

impl<T> LvmthinClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "lvmthin"),
        }
    }
}
impl<T> LvmthinClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List local LVM Thin Pools."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncLvmthinClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncLvmthinClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "lvmthin"),
        }
    }
}
impl<T> AsyncLvmthinClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List local LVM Thin Pools."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
