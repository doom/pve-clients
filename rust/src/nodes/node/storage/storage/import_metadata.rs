#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetParameters {
    #[doc = "Volume identifier for the guest archive/entry."]
    pub volume: String,
}

#[doc = "Information about how to import a guest."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Parameters which can be used in a call to create a VM or container."]
    #[serde(rename = "create-args")]
    pub create_args: CreateArgs,
    #[doc = "Recognised disk volumes as `$bus$id` => `$storeid:$path` map."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub disks: Option<Disks>,
    #[doc = "Recognised network interfaces as `net$id` => { ...params } object."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub net: Option<Net>,
    #[doc = "The type of the import-source of this guest volume."]
    pub source: String,
    #[doc = "The type of guest this is going to produce."]
    pub r#type: String,
    #[doc = "List of known issues that can affect the import of a guest. Note that lack of warning does not imply that there cannot be any problems."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub warnings: Option<Vec<GetResponseWarningsItem>>,
}

impl GetResponseItem {
    pub fn new(create_args: CreateArgs, source: String, r#type: String) -> Self {
        Self {
            create_args,
            source,
            r#type,
            disks: Default::default(),
            net: Default::default(),
            warnings: Default::default(),
        }
    }
}

#[doc = "Parameters which can be used in a call to create a VM or container."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CreateArgs {}

#[doc = "Recognised disk volumes as `$bus$id` => `$storeid:$path` map."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Disks {}

#[doc = "Recognised network interfaces as `net$id` => { ...params } object."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Net {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseWarningsItem {
    #[doc = "Related subject (config) key of warning."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub key: Option<String>,
    #[doc = "What this warning is about."]
    pub r#type: String,
    #[doc = "Related subject (config) value of warning."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub value: Option<String>,
}

impl GetResponseWarningsItem {
    pub fn new(r#type: String) -> Self {
        Self {
            r#type,
            key: Default::default(),
            value: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImportMetadataClient<T> {
    client: T,
    path: String,
}

impl<T> ImportMetadataClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "import-metadata"),
        }
    }
}
impl<T> ImportMetadataClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get the base parameters for creating a guest which imports data from a foreign importable guest, like an ESXi VM"]
    pub fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncImportMetadataClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncImportMetadataClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "import-metadata"),
        }
    }
}
impl<T> AsyncImportMetadataClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get the base parameters for creating a guest which imports data from a foreign importable guest, like an ESXi VM"]
    pub async fn get(&self, parameters: GetParameters) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
