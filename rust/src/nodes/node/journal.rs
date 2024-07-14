#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetParameters {
    #[doc = "End before the given Cursor. Conflicts with 'until'"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub endcursor: Option<String>,
    #[doc = "Limit to the last X lines. Conflicts with a range."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lastentries: Option<u64>,
    #[doc = "Display all log since this UNIX epoch. Conflicts with 'startcursor'."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub since: Option<u64>,
    #[doc = "Start after the given Cursor. Conflicts with 'since'"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub startcursor: Option<String>,
    #[doc = "Display all log until this UNIX epoch. Conflicts with 'endcursor'."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub until: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct JournalClient<T> {
    client: T,
    path: String,
}

impl<T> JournalClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "journal"),
        }
    }
}
impl<T> JournalClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Read Journal"]
    pub fn get(&self, parameters: GetParameters) -> Result<Vec<String>, T::Error> {
        self.client.get(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncJournalClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncJournalClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "journal"),
        }
    }
}
impl<T> AsyncJournalClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Read Journal"]
    pub async fn get(&self, parameters: GetParameters) -> Result<Vec<String>, T::Error> {
        self.client.get(&self.path, &parameters).await
    }
}
