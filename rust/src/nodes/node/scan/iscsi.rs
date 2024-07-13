#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "The iSCSI portal (IP or DNS name with optional port)."]
    pub portal: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "The iSCSI portal name."]
    pub portal: String,
    #[doc = "The iSCSI target name."]
    pub target: String,
}

#[derive(Debug, Clone)]
pub struct IscsiClient<T> {
    client: T,
    path: String,
}

impl<T> IscsiClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "iscsi"),
        }
    }
}
impl<T> IscsiClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Scan remote iSCSI server."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncIscsiClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncIscsiClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "iscsi"),
        }
    }
}
impl<T> AsyncIscsiClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Scan remote iSCSI server."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
