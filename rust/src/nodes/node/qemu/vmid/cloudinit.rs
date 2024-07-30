pub mod dump;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Indicates a pending delete request if present and not 0. "]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<u64>,
    #[doc = "Configuration option name."]
    pub key: String,
    #[doc = "The new pending value."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pending: Option<String>,
    #[doc = "Value as it was used to generate the current cloudinit image."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CloudinitClient<T> {
    client: T,
    path: String,
}

impl<T> CloudinitClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "cloudinit"),
        }
    }

    pub fn dump(&self) -> dump::DumpClient<T> {
        dump::DumpClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CloudinitClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get the cloudinit configuration with both current and pending values."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Regenerate and change cloudinit config drive."]
    pub fn put(&self) -> Result<(), T::Error> {
        self.client.put(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCloudinitClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCloudinitClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "cloudinit"),
        }
    }

    pub fn dump(&self) -> dump::AsyncDumpClient<T> {
        dump::AsyncDumpClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncCloudinitClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get the cloudinit configuration with both current and pending values."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Regenerate and change cloudinit config drive."]
    pub async fn put(&self) -> Result<(), T::Error> {
        self.client.put(&self.path, &()).await
    }
}
