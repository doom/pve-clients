pub mod ipam;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "Only list sdn ipams of specific type"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub ipam: String,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "The SDN ipam object identifier."]
    pub ipam: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub section: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub token: Option<String>,
    #[doc = "Plugin type."]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
}

impl PostParameters {
    pub fn new(ipam: String, r#type: String) -> Self {
        Self {
            ipam,
            r#type,
            section: Default::default(),
            token: Default::default(),
            url: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpamsClient<T> {
    client: T,
    path: String,
}

impl<T> IpamsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "ipams"),
        }
    }

    pub fn ipam(&self, ipam: &str) -> ipam::IpamClient<T> {
        ipam::IpamClient::<T>::new(self.client.clone(), &self.path, ipam)
    }
}
impl<T> IpamsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "SDN ipams index."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Create a new sdn ipam object."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncIpamsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncIpamsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "ipams"),
        }
    }

    pub fn ipam(&self, ipam: &str) -> ipam::AsyncIpamClient<T> {
        ipam::AsyncIpamClient::<T>::new(self.client.clone(), &self.path, ipam)
    }
}
impl<T> AsyncIpamsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "SDN ipams index."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Create a new sdn ipam object."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
