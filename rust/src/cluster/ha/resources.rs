pub mod sid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "Only list resources of specific type"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub sid: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Description."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "The HA group identifier."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub group: Option<String>,
    #[doc = "Maximal number of service relocate tries when a service failes to start."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_relocate: Option<u64>,
    #[doc = "Maximal number of tries to restart the service on a node after its start failed."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_restart: Option<u64>,
    #[doc = "HA resource ID. This consists of a resource type followed by a resource specific name, separated with colon (example: vm:100 / ct:100). For virtual machines and containers, you can simply use the VM or CT id as a shortcut (example: 100)."]
    pub sid: String,
    #[doc = "Requested resource state."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub state: Option<String>,
    #[doc = "Resource type."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
}

impl PostParameters {
    pub fn new(sid: String) -> Self {
        Self {
            sid,
            comment: Default::default(),
            group: Default::default(),
            max_relocate: Default::default(),
            max_restart: Default::default(),
            state: Default::default(),
            r#type: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResourcesClient<T> {
    client: T,
    path: String,
}

impl<T> ResourcesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "resources"),
        }
    }

    pub fn sid(&self, sid: &str) -> sid::SidClient<T> {
        sid::SidClient::<T>::new(self.client.clone(), &self.path, sid)
    }
}
impl<T> ResourcesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List HA resources."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }

    #[doc = "Create a new HA resource."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncResourcesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncResourcesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "resources"),
        }
    }

    pub fn sid(&self, sid: &str) -> sid::AsyncSidClient<T> {
        sid::AsyncSidClient::<T>::new(self.client.clone(), &self.path, sid)
    }
}
impl<T> AsyncResourcesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List HA resources."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }

    #[doc = "Create a new HA resource."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
