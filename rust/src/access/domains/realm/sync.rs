#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "If set, does not write anything."]
    #[serde(
        rename = "dry-run",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub dry_run: Option<bool>,
    #[doc = "Enable newly synced users immediately."]
    #[serde(
        rename = "enable-new",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enable_new: Option<bool>,
    #[doc = "DEPRECATED: use 'remove-vanished' instead. If set, uses the LDAP Directory as source of truth, deleting users or groups not returned from the sync and removing all locally modified properties of synced users. If not set, only syncs information which is present in the synced data, and does not delete or modify anything else."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub full: Option<bool>,
    #[doc = "DEPRECATED: use 'remove-vanished' instead. Remove ACLs for users or groups which were removed from the config during a sync."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub purge: Option<bool>,
    #[doc = "A semicolon-seperated list of things to remove when they or the user vanishes during a sync. The following values are possible: 'entry' removes the user/group when not returned from the sync. 'properties' removes the set properties on existing user/group that do not appear in the source (even custom ones). 'acl' removes acls when the user/group is not returned from the sync. Instead of a list it also can be 'none' (the default)."]
    #[serde(
        rename = "remove-vanished",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub remove_vanished: Option<String>,
    #[doc = "Select what to sync."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub scope: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SyncClient<T> {
    client: T,
    path: String,
}

impl<T> SyncClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "sync"),
        }
    }
}
impl<T> SyncClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Syncs users and/or groups from the configured LDAP to user.cfg. NOTE: Synced groups will have the name 'name-$realm', so make sure those groups do not exist to prevent overwriting."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncSyncClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncSyncClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "sync"),
        }
    }
}
impl<T> AsyncSyncClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Syncs users and/or groups from the configured LDAP to user.cfg. NOTE: Synced groups will have the name 'name-$realm', so make sure those groups do not exist to prevent overwriting."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
