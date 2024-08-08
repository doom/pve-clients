#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Meta-information about the boot mode."]
    #[serde(rename = "boot-info")]
    pub boot_info: BootInfo,
    #[doc = "The uptime of the system in seconds."]
    #[serde(rename = "current-kernel")]
    pub current_kernel: CurrentKernel,
}

#[doc = "Meta-information about the boot mode."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BootInfo {
    #[doc = "Through which firmware the system got booted."]
    pub mode: String,
    #[doc = "System is booted in secure mode, only applicable for the \"efi\" mode."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub secureboot: Option<bool>,
}

impl BootInfo {
    pub fn new(mode: String) -> Self {
        Self {
            mode,
            secureboot: Default::default(),
        }
    }
}

#[doc = "The uptime of the system in seconds."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CurrentKernel {
    #[doc = "Hardware (architecture) type"]
    pub machine: String,
    #[doc = "OS kernel release (e.g., \"6.8.0\")"]
    pub release: String,
    #[doc = "OS kernel name (e.g., \"Linux\")"]
    pub sysname: String,
    #[doc = "OS kernel version with build info"]
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Specify the command."]
    pub command: String,
}

#[derive(Debug, Clone)]
pub struct StatusClient<T> {
    client: T,
    path: String,
}

impl<T> StatusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "status"),
        }
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Read node status"]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Reboot or shutdown a node."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncStatusClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncStatusClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "status"),
        }
    }
}
impl<T> AsyncStatusClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Read node status"]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Reboot or shutdown a node."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
