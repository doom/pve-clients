#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Comment"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Disable this matcher"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub disable: Option<bool>,
    #[doc = "Invert match of the whole matcher"]
    #[serde(
        rename = "invert-match",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub invert_match: Option<bool>,
    #[doc = "Match notification timestamp"]
    #[serde(
        rename = "match-calendar",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub match_calendar: Option<Vec<String>>,
    #[doc = "Metadata fields to match (regex or exact match). Must be in the form (regex|exact):<field>=<value>"]
    #[serde(
        rename = "match-field",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub match_field: Option<Vec<String>>,
    #[doc = "Notification severities to match"]
    #[serde(
        rename = "match-severity",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub match_severity: Option<Vec<String>>,
    #[doc = "Choose between 'all' and 'any' for when multiple properties are specified"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mode: Option<String>,
    #[doc = "Name of the matcher."]
    pub name: String,
    #[doc = "Targets to notify on match"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target: Option<Vec<String>>,
}

impl GetResponseItem {
    pub fn new(name: String) -> Self {
        Self {
            name,
            comment: Default::default(),
            digest: Default::default(),
            disable: Default::default(),
            invert_match: Default::default(),
            match_calendar: Default::default(),
            match_field: Default::default(),
            match_severity: Default::default(),
            mode: Default::default(),
            target: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PutParameters {
    #[doc = "Comment"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "A list of settings you want to delete."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delete: Option<Vec<String>>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<String>,
    #[doc = "Disable this matcher"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub disable: Option<bool>,
    #[doc = "Invert match of the whole matcher"]
    #[serde(
        rename = "invert-match",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub invert_match: Option<bool>,
    #[doc = "Match notification timestamp"]
    #[serde(
        rename = "match-calendar",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub match_calendar: Option<Vec<String>>,
    #[doc = "Metadata fields to match (regex or exact match). Must be in the form (regex|exact):<field>=<value>"]
    #[serde(
        rename = "match-field",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub match_field: Option<Vec<String>>,
    #[doc = "Notification severities to match"]
    #[serde(
        rename = "match-severity",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub match_severity: Option<Vec<String>>,
    #[doc = "Choose between 'all' and 'any' for when multiple properties are specified"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mode: Option<String>,
    #[doc = "Targets to notify on match"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct NameClient<T> {
    client: T,
    path: String,
}

impl<T> NameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
impl<T> NameClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Remove matcher"]
    pub fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &())
    }

    #[doc = "Return a specific matcher"]
    pub fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Update existing matcher"]
    pub fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncNameClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncNameClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
impl<T> AsyncNameClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Remove matcher"]
    pub async fn delete(&self) -> Result<(), T::Error> {
        self.client.delete(&self.path, &()).await
    }

    #[doc = "Return a specific matcher"]
    pub async fn get(&self) -> Result<GetResponseItem, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Update existing matcher"]
    pub async fn put(&self, parameters: PutParameters) -> Result<(), T::Error> {
        self.client.put(&self.path, &parameters).await
    }
}
