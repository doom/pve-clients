pub mod id;

#[doc = "TFA Entry."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Creation time of this entry as unix epoch."]
    pub created: u64,
    #[doc = "User chosen description for this entry."]
    pub description: String,
    #[doc = "Whether this TFA entry is currently enabled."]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub enable: Option<bool>,
    #[doc = "The id used to reference this entry."]
    pub id: String,
    #[doc = "TFA Entry Type."]
    pub r#type: String,
}

impl GetResponseItem {
    pub fn new(created: u64, description: String, id: String, r#type: String) -> Self {
        Self {
            created,
            description,
            id,
            r#type,
            enable: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "When responding to a u2f challenge: the original challenge string"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub challenge: Option<String>,
    #[doc = "A description to distinguish multiple entries from one another"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[doc = "The current password of the user performing the change."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub password: Option<String>,
    #[doc = "A totp URI."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub totp: Option<String>,
    #[doc = "TFA Entry Type."]
    pub r#type: String,
    #[doc = "The current value for the provided totp URI, or a Webauthn/U2F challenge response"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub value: Option<String>,
}

impl PostParameters {
    pub fn new(r#type: String) -> Self {
        Self {
            r#type,
            challenge: Default::default(),
            description: Default::default(),
            password: Default::default(),
            totp: Default::default(),
            value: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {
    #[doc = "When adding u2f entries, this contains a challenge the user must respond to in order to finish the registration."]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub challenge: Option<String>,
    #[doc = "The id of a newly added TFA entry."]
    pub id: String,
    #[doc = "When adding recovery codes, this contains the list of codes to be displayed to the user"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub recovery: Option<Vec<String>>,
}

impl PostResponseItem {
    pub fn new(id: String) -> Self {
        Self {
            id,
            challenge: Default::default(),
            recovery: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UseridClient<T> {
    client: T,
    path: String,
}

impl<T> UseridClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, userid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, userid),
        }
    }

    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> UseridClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List TFA configurations of users."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Add a TFA entry for a user."]
    pub fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncUseridClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncUseridClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str, userid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, userid),
        }
    }

    pub fn id(&self, id: &str) -> id::AsyncIdClient<T> {
        id::AsyncIdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
impl<T> AsyncUseridClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List TFA configurations of users."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Add a TFA entry for a user."]
    pub async fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
