pub mod id;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Description."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Flag to disable/deactivate the entry."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub disable: Option<bool>,
    #[doc = "Replication Job ID. The ID is composed of a Guest ID and a job number, separated by a hyphen, i.e. '<GUEST>-<JOBNUM>'."]
    pub id: String,
    #[doc = "Rate limit in mbps (megabytes per second) as floating point number."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rate: Option<f64>,
    #[doc = "Mark the replication job for removal. The job will remove all local replication snapshots. When set to 'full', it also tries to remove replicated volumes on the target. The job then removes itself from the configuration file."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub remove_job: Option<String>,
    #[doc = "Storage replication schedule. The format is a subset of `systemd` calendar events."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub schedule: Option<String>,
    #[doc = "For internal use, to detect if the guest was stolen."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub source: Option<String>,
    #[doc = "Target node."]
    pub target: String,
    #[doc = "Section type."]
    pub r#type: String,
}

#[derive(Debug, Clone)]
pub struct ReplicationClient<T> {
    client: T,
    path: String,
}

impl<T> ReplicationClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "replication"),
        }
    }

    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> ReplicationClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List replication jobs."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create a new replication job"]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncReplicationClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncReplicationClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "replication"),
        }
    }

    pub fn id(&self, id: &str) -> id::AsyncIdClient<T> {
        id::AsyncIdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> AsyncReplicationClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List replication jobs."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create a new replication job"]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
