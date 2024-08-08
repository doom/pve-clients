pub mod snapname;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Snapshot description."]
    pub description: String,
    #[doc = "Snapshot identifier. Value 'current' identifies the current VM."]
    pub name: String,
    #[doc = "Parent snapshot identifier."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub parent: Option<String>,
    #[doc = "Snapshot creation time"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub snaptime: Option<u64>,
    #[doc = "Snapshot includes RAM."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vmstate: Option<bool>,
}

impl GetResponseItem {
    pub fn new(description: String, name: String) -> Self {
        Self {
            description,
            name,
            parent: Default::default(),
            snaptime: Default::default(),
            vmstate: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "A textual description or comment."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[doc = "The name of the snapshot."]
    pub snapname: String,
    #[doc = "Save the vmstate"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub vmstate: Option<bool>,
}

impl PostParameters {
    pub fn new(snapname: String) -> Self {
        Self {
            snapname,
            description: Default::default(),
            vmstate: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotClient<T> {
    client: T,
    path: String,
}

impl<T> SnapshotClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "snapshot"),
        }
    }

    pub fn snapname(&self, snapname: &str) -> snapname::SnapnameClient<T> {
        snapname::SnapnameClient::<T>::new(self.client.clone(), &self.path, snapname)
    }
}
impl<T> SnapshotClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List all snapshots."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Snapshot a VM."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSnapshotClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSnapshotClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "snapshot"),
        }
    }

    pub fn snapname(&self, snapname: &str) -> snapname::AsyncSnapnameClient<T> {
        snapname::AsyncSnapnameClient::<T>::new(self.client.clone(), &self.path, snapname)
    }
}
impl<T> AsyncSnapshotClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List all snapshots."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Snapshot a VM."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
