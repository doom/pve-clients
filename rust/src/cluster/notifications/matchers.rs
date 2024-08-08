pub mod name;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Comment"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
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
    #[doc = "Show if this entry was created by a user or was built-in"]
    pub origin: String,
    #[doc = "Targets to notify on match"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target: Option<Vec<String>>,
}

impl GetResponseItem {
    pub fn new(name: String, origin: String) -> Self {
        Self {
            name,
            origin,
            comment: Default::default(),
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "Comment"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
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

impl PostParameters {
    pub fn new(name: String) -> Self {
        Self {
            name,
            comment: Default::default(),
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

#[derive(Debug, Clone)]
pub struct MatchersClient<T> {
    client: T,
    path: String,
}

impl<T> MatchersClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "matchers"),
        }
    }

    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> MatchersClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Returns a list of all matchers"]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create a new matcher"]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncMatchersClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncMatchersClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "matchers"),
        }
    }

    pub fn name(&self, name: &str) -> name::AsyncNameClient<T> {
        name::AsyncNameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> AsyncMatchersClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Returns a list of all matchers"]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create a new matcher"]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
