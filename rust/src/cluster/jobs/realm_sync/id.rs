#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Description for the Job."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Enable newly synced users immediately."]
    #[serde(
        rename = "enable-new",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enable_new: Option<bool>,
    #[doc = "Determines if the job is enabled."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enabled: Option<bool>,
    #[doc = "Authentication domain ID"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub realm: Option<String>,
    #[doc = "A semicolon-seperated list of things to remove when they or the user vanishes during a sync. The following values are possible: 'entry' removes the user/group when not returned from the sync. 'properties' removes the set properties on existing user/group that do not appear in the source (even custom ones). 'acl' removes acls when the user/group is not returned from the sync. Instead of a list it also can be 'none' (the default)."]
    #[serde(
        rename = "remove-vanished",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub remove_vanished: Option<String>,
    #[doc = "Backup schedule. The format is a subset of `systemd` calendar events."]
    pub schedule: String,
    #[doc = "Select what to sync."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PutParameters {
    #[doc = "Description for the Job."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<String>,
    #[doc = "Enable newly synced users immediately."]
    #[serde(
        rename = "enable-new",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enable_new: Option<bool>,
    #[doc = "Determines if the job is enabled."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enabled: Option<bool>,
    #[doc = "A semicolon-seperated list of things to remove when they or the user vanishes during a sync. The following values are possible: 'entry' removes the user/group when not returned from the sync. 'properties' removes the set properties on existing user/group that do not appear in the source (even custom ones). 'acl' removes acls when the user/group is not returned from the sync. Instead of a list it also can be 'none' (the default)."]
    #[serde(
        rename = "remove-vanished",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub remove_vanished: Option<String>,
    #[doc = "Backup schedule. The format is a subset of `systemd` calendar events."]
    pub schedule: String,
    #[doc = "Select what to sync."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub scope: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IdClient<T> {
    client: T,
    path: String,
}

impl<T> IdClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, id),
        }
    }
}
impl<T> IdClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Delete realm-sync job definition."]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Read realm-sync job definition."]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create new realm-sync job."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }

    #[doc = "Update realm-sync job definition."]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncIdClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncIdClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, id),
        }
    }
}
impl<T> AsyncIdClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Delete realm-sync job definition."]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Read realm-sync job definition."]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create new realm-sync job."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }

    #[doc = "Update realm-sync job definition."]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
