#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bwlimit: Option<u64>,
    #[doc = "Description for the new VM."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[doc = "Target format for file storage. Only valid for full clone."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub format: Option<String>,
    #[doc = "Create a full copy of all disks. This is always done when you clone a normal VM. For VM templates, we try to create a linked clone by default."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub full: Option<bool>,
    #[doc = "Set a name for the new VM."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    #[doc = "VMID for the clone."]
    pub newid: u64,
    #[doc = "Add the new VM to the specified pool."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pool: Option<String>,
    #[doc = "The name of the snapshot."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub snapname: Option<String>,
    #[doc = "Target storage for full clone."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub storage: Option<String>,
    #[doc = "Target node. Only allowed if the original VM is on shared storage."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target: Option<String>,
}

impl PostParameters {
    pub fn new(newid: u64) -> Self {
        Self {
            newid,
            bwlimit: Default::default(),
            description: Default::default(),
            format: Default::default(),
            full: Default::default(),
            name: Default::default(),
            pool: Default::default(),
            snapname: Default::default(),
            storage: Default::default(),
            target: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CloneClient<T> {
    client: T,
    path: String,
}

impl<T> CloneClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "clone"),
        }
    }
}
impl<T> CloneClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Create a copy of virtual machine/template."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCloneClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCloneClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "clone"),
        }
    }
}
impl<T> AsyncCloneClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Create a copy of virtual machine/template."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
