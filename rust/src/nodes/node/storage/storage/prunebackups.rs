#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct DeleteParameters {
    #[doc = "Use these retention options instead of those from the storage configuration."]
    #[serde(
        rename = "prune-backups",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub prune_backups: Option<String>,
    #[doc = "Either 'qemu' or 'lxc'. Only consider backups for guests of this type."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
    #[doc = "Only prune backups for this VM."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmid: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "Use these retention options instead of those from the storage configuration."]
    #[serde(
        rename = "prune-backups",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub prune_backups: Option<String>,
    #[doc = "Either 'qemu' or 'lxc'. Only consider backups for guests of this type."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub r#type: Option<String>,
    #[doc = "Only consider backups for this guest."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmid: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Creation time of the backup (seconds since the UNIX epoch)."]
    pub ctime: u64,
    #[doc = "Whether the backup would be kept or removed. Backups that are protected or don't use the standard naming scheme are not removed."]
    pub mark: String,
    #[doc = "One of 'qemu', 'lxc', 'openvz' or 'unknown'."]
    pub r#type: String,
    #[doc = "The VM the backup belongs to."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmid: Option<u64>,
    #[doc = "Backup volume ID."]
    pub volid: String,
}

impl GetResponseItem {
    pub fn new(ctime: u64, mark: String, r#type: String, volid: String) -> Self {
        Self {
            ctime,
            mark,
            r#type,
            volid,
            vmid: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrunebackupsClient<T> {
    client: T,
    path: String,
}

impl<T> PrunebackupsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "prunebackups"),
        }
    }
}
impl<T> PrunebackupsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Prune backups. Only those using the standard naming scheme are considered."]
    pub fn delete(&self, parameters: DeleteParameters) -> Result<String, T::Error> {
        self.client.delete(&self.path, &parameters)
    }

    #[doc = "Get prune information for backups. NOTE: this is only a preview and might not be what a subsequent prune call does if backups are removed/added in the meantime."]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncPrunebackupsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncPrunebackupsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "prunebackups"),
        }
    }
}
impl<T> AsyncPrunebackupsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Prune backups. Only those using the standard naming scheme are considered."]
    pub async fn delete(&self, parameters: DeleteParameters) -> Result<String, T::Error> {
        self.client.delete(&self.path, &parameters).await
    }

    #[doc = "Get prune information for backups. NOTE: this is only a preview and might not be what a subsequent prune call does if backups are removed/added in the meantime."]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
