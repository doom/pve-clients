#[doc = "Root node of the tree object. Children represent guests, grandchildren represent volumes of that guest."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub children: Vec<GetResponseChildrenItem>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseChildrenItem {
    #[doc = "The volumes of the guest with the information if they will be included in backups."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub children: Option<Vec<GetResponseChildrenItemChildrenItem>>,
    #[doc = "VMID of the guest."]
    pub id: u64,
    #[doc = "Name of the guest"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    #[doc = "Type of the guest, VM, CT or unknown for removed but not purged guests."]
    pub r#type: String,
}

impl GetResponseChildrenItem {
    pub fn new(id: u64, r#type: String) -> Self {
        Self {
            id,
            r#type,
            children: Default::default(),
            name: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseChildrenItemChildrenItem {
    #[doc = "Configuration key of the volume."]
    pub id: String,
    #[doc = "Whether the volume is included in the backup or not."]
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub included: bool,
    #[doc = "Name of the volume."]
    pub name: String,
    #[doc = "The reason why the volume is included (or excluded)."]
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct IncludedVolumesClient<T> {
    client: T,
    path: String,
}

impl<T> IncludedVolumesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "included_volumes"),
        }
    }
}
impl<T> IncludedVolumesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Returns included guests and the backup status of their disks. Optimized to be used in ExtJS tree views."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncIncludedVolumesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncIncludedVolumesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "included_volumes"),
        }
    }
}
impl<T> AsyncIncludedVolumesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Returns included guests and the backup status of their disks. Optimized to be used in ExtJS tree views."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
