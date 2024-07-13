pub mod id;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Flag to disable the plugin."]
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub disable: bool,
    #[doc = "The ID of the entry."]
    pub id: String,
    #[doc = "Server network port"]
    pub port: u64,
    #[doc = "Server dns name or IP address"]
    pub server: String,
    #[doc = "Plugin type."]
    pub r#type: String,
}

#[derive(Debug, Clone)]
pub struct ServerClient<T> {
    client: T,
    path: String,
}

impl<T> ServerClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "server"),
        }
    }

    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> ServerClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List configured metric servers."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncServerClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncServerClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "server"),
        }
    }

    pub fn id(&self, id: &str) -> id::AsyncIdClient<T> {
        id::AsyncIdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> AsyncServerClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List configured metric servers."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
