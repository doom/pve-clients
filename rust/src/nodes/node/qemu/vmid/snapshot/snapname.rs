pub mod config;
pub mod rollback;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct DeleteParameters {
    #[doc = "For removal from config file, even if removing disk snapshots fails."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub force: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct SnapnameClient<T> {
    client: T,
    path: String,
}

impl<T> SnapnameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, snapname: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, snapname),
        }
    }

    pub fn config(&self) -> config::ConfigClient<T> {
        config::ConfigClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rollback(&self) -> rollback::RollbackClient<T> {
        rollback::RollbackClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> SnapnameClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete a VM snapshot."]
    pub fn delete(&self, parameters: DeleteParameters) -> Result<String, T::Error> {
        self.client.delete(&self.path, &parameters)
    }

    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSnapnameClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSnapnameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, snapname: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, snapname),
        }
    }

    pub fn config(&self) -> config::AsyncConfigClient<T> {
        config::AsyncConfigClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rollback(&self) -> rollback::AsyncRollbackClient<T> {
        rollback::AsyncRollbackClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncSnapnameClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete a VM snapshot."]
    pub async fn delete(&self, parameters: DeleteParameters) -> Result<String, T::Error> {
        self.client.delete(&self.path, &parameters).await
    }

    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
