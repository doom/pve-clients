#[doc = "Result from parsing the APT repository files in /etc/apt/."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Common digest of all files."]
    pub digest: String,
    #[doc = "List of problematic repository files."]
    pub errors: Vec<GetResponseErrorsItem>,
    #[doc = "List of parsed repository files."]
    pub files: Vec<GetResponseFilesItem>,
    #[doc = "Additional information/warnings for APT repositories."]
    pub infos: Vec<GetResponseInfosItem>,
    #[doc = "List of standard repositories and their configuration status"]
    #[serde(rename = "standard-repos")]
    pub standard_repos: Vec<GetResponseStandardReposItem>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseErrorsItem {
    #[doc = "The error message"]
    pub error: String,
    #[doc = "Path to the problematic file."]
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseFilesItem {
    #[doc = "Digest of the file as bytes."]
    pub digest: Vec<u64>,
    #[doc = "Format of the file."]
    #[serde(rename = "file-type")]
    pub file_type: String,
    #[doc = "Path to the problematic file."]
    pub path: String,
    #[doc = "The parsed repositories."]
    pub repositories: Vec<GetResponseFilesItemRepositoriesItem>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseFilesItemRepositoriesItem {
    #[doc = "Associated comment"]
    #[serde(rename = "Comment", skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "List of repository components"]
    #[serde(
        rename = "Components",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub components: Option<Vec<String>>,
    #[doc = "Whether the repository is enabled or not"]
    #[serde(
        rename = "Enabled",
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub enabled: bool,
    #[doc = "Format of the defining file."]
    #[serde(rename = "FileType")]
    pub file_type: String,
    #[doc = "Additional options"]
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none", default)]
    pub options: Option<Vec<GetResponseFilesItemRepositoriesItemOptionsItem>>,
    #[doc = "List of package distribuitions"]
    #[serde(rename = "Suites")]
    pub suites: Vec<String>,
    #[doc = "List of package types."]
    #[serde(rename = "Types")]
    pub types: Vec<String>,
    #[doc = "List of repository URIs."]
    #[serde(rename = "URIs")]
    pub uris: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseFilesItemRepositoriesItemOptionsItem {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseInfosItem {
    #[doc = "Index of the associated repository within the file."]
    pub index: String,
    #[doc = "Kind of the information (e.g. warning)."]
    pub kind: String,
    #[doc = "Information message."]
    pub message: String,
    #[doc = "Path to the associated file."]
    pub path: String,
    #[doc = "Property from which the info originates."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub property: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseStandardReposItem {
    #[doc = "Handle to identify the repository."]
    pub handle: String,
    #[doc = "Full name of the repository."]
    pub name: String,
    #[doc = "Indicating enabled/disabled status, if the repository is configured."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub status: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Digest to detect modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Whether the repository should be enabled or not."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enabled: Option<bool>,
    #[doc = "Index within the file (starting from 0)."]
    pub index: u64,
    #[doc = "Path to the containing file."]
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "Digest to detect modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Handle that identifies a repository."]
    pub handle: String,
}

#[derive(Debug, Clone)]
pub struct RepositoriesClient<T> {
    client: T,
    path: String,
}

impl<T> RepositoriesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "repositories"),
        }
    }
}
impl<T> RepositoriesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get APT repository information."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Change the properties of a repository. Currently only allows enabling/disabling."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }

    #[doc = "Add a standard repository to the configuration"]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncRepositoriesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncRepositoriesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "repositories"),
        }
    }
}
impl<T> AsyncRepositoriesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get APT repository information."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Change the properties of a repository. Currently only allows enabling/disabling."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }

    #[doc = "Add a standard repository to the configuration"]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
