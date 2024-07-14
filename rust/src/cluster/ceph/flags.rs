pub mod flag;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Flag description."]
    pub description: String,
    #[doc = "Flag name."]
    pub name: String,
    #[doc = "Flag value."]
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "Backfilling of PGs is suspended."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub nobackfill: Option<bool>,
    #[doc = "Deep Scrubbing is disabled."]
    #[serde(
        rename = "nodeep-scrub",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub nodeep_scrub: Option<bool>,
    #[doc = "OSD failure reports are being ignored, such that the monitors will not mark OSDs down."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub nodown: Option<bool>,
    #[doc = "OSDs that were previously marked out will not be marked back in when they start."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub noin: Option<bool>,
    #[doc = "OSDs will not automatically be marked out after the configured interval."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub noout: Option<bool>,
    #[doc = "Rebalancing of PGs is suspended."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub norebalance: Option<bool>,
    #[doc = "Recovery of PGs is suspended."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub norecover: Option<bool>,
    #[doc = "Scrubbing is disabled."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub noscrub: Option<bool>,
    #[doc = "Cache tiering activity is suspended."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub notieragent: Option<bool>,
    #[doc = "OSDs are not allowed to start."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub noup: Option<bool>,
    #[doc = "Pauses read and writes."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub pause: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct FlagsClient<T> {
    client: T,
    path: String,
}

impl<T> FlagsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "flags"),
        }
    }

    pub fn flag(&self, flag: &str) -> flag::FlagClient<T> {
        flag::FlagClient::<T>::new(self.client.clone(), &self.path, flag)
    }
}
impl<T> FlagsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "get the status of all ceph flags"]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Set/Unset multiple ceph flags at once."]
    pub fn put(&self, parameters: PutParameters) -> Result<String, T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncFlagsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncFlagsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "flags"),
        }
    }

    pub fn flag(&self, flag: &str) -> flag::AsyncFlagClient<T> {
        flag::AsyncFlagClient::<T>::new(self.client.clone(), &self.path, flag)
    }
}
impl<T> AsyncFlagsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "get the status of all ceph flags"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Set/Unset multiple ceph flags at once."]
    pub async fn put(&self, parameters: PutParameters) -> Result<String, T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
