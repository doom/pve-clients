#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The LVM logical volume group name."]
    pub vg: String,
}

#[derive(Debug, Clone)]
pub struct LvmClient<T> {
    client: T,
    path: String,
}

impl<T> LvmClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "lvm"),
        }
    }
}
impl<T> LvmClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List local LVM volume groups."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncLvmClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncLvmClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "lvm"),
        }
    }
}
impl<T> AsyncLvmClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List local LVM volume groups."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
