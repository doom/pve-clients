pub mod id;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "A comment for the job."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "If the job is enabled or not."]
    #[serde(
        deserialize_with = "crate::common::deserialize_bool_lax",
        serialize_with = "crate::common::serialize_bool_as_u64"
    )]
    pub enabled: bool,
    #[doc = "The ID of the entry."]
    pub id: String,
    #[doc = "Last execution time of the job in seconds since the beginning of the UNIX epoch"]
    #[serde(rename = "last-run", skip_serializing_if = "Option::is_none", default)]
    pub last_run: Option<u64>,
    #[doc = "Next planned execution time of the job in seconds since the beginning of the UNIX epoch."]
    #[serde(rename = "next-run", skip_serializing_if = "Option::is_none", default)]
    pub next_run: Option<u64>,
    #[doc = "Authentication domain ID"]
    pub realm: String,
    #[doc = "A semicolon-seperated list of things to remove when they or the user vanishes during a sync. The following values are possible: 'entry' removes the user/group when not returned from the sync. 'properties' removes the set properties on existing user/group that do not appear in the source (even custom ones). 'acl' removes acls when the user/group is not returned from the sync. Instead of a list it also can be 'none' (the default)."]
    #[serde(
        rename = "remove-vanished",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub remove_vanished: Option<String>,
    #[doc = "The configured sync schedule."]
    pub schedule: String,
    #[doc = "Select what to sync."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub scope: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RealmSyncClient<T> {
    client: T,
    path: String,
}

impl<T> RealmSyncClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "realm-sync"),
        }
    }

    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> RealmSyncClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List configured realm-sync-jobs."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncRealmSyncClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncRealmSyncClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "realm-sync"),
        }
    }

    pub fn id(&self, id: &str) -> id::AsyncIdClient<T> {
        id::AsyncIdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> AsyncRealmSyncClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List configured realm-sync-jobs."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
