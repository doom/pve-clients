pub mod migrate;
pub mod relocate;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Description."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Can be used to prevent concurrent modifications."]
    pub digest: String,
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
    #[doc = "The type of the resources."]
    pub r#type: String,
}

impl GetResponseItem {
    pub fn new(digest: String, sid: String, r#type: String) -> Self {
        Self {
            digest,
            sid,
            r#type,
            comment: Default::default(),
            group: Default::default(),
            max_relocate: Default::default(),
            max_restart: Default::default(),
            state: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "Description."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "The HA group identifier."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub group: Option<String>,
    #[doc = "Maximal number of service relocate tries when a service failes to start."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_relocate: Option<u64>,
    #[doc = "Maximal number of tries to restart the service on a node after its start failed."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub max_restart: Option<u64>,
    #[doc = "Requested resource state."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub state: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SidClient<T> {
    client: T,
    path: String,
}

impl<T> SidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, sid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, sid),
        }
    }

    pub fn migrate(&self) -> migrate::MigrateClient<T> {
        migrate::MigrateClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn relocate(&self) -> relocate::RelocateClient<T> {
        relocate::RelocateClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> SidClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete resource configuration."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Read resource configuration."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update resource configuration."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSidClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, sid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, sid),
        }
    }

    pub fn migrate(&self) -> migrate::AsyncMigrateClient<T> {
        migrate::AsyncMigrateClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn relocate(&self) -> relocate::AsyncRelocateClient<T> {
        relocate::AsyncRelocateClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncSidClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete resource configuration."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Read resource configuration."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update resource configuration."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
