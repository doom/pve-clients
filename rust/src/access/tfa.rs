pub mod userid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub entries: Vec<GetResponseItemEntriesItem>,
    #[doc = "User this entry belongs to."]
    pub userid: String,
}

#[doc = "TFA Entry."]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItemEntriesItem {
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "The response to the current authentication challenge."]
    pub response: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponseItem {
    pub ticket: String,
}

#[derive(Debug, Clone)]
pub struct TfaClient<T> {
    client: T,
    path: String,
}

impl<T> TfaClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "tfa"),
        }
    }

    pub fn userid(&self, userid: &str) -> userid::UseridClient<T> {
        userid::UseridClient::<T>::new(self.client.clone(), &self.path, userid)
    }
}
impl<T> TfaClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List TFA configurations of users."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Finish a u2f challenge."]
    pub fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncTfaClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncTfaClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "tfa"),
        }
    }

    pub fn userid(&self, userid: &str) -> userid::AsyncUseridClient<T> {
        userid::AsyncUseridClient::<T>::new(self.client.clone(), &self.path, userid)
    }
}
impl<T> AsyncTfaClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List TFA configurations of users."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Finish a u2f challenge."]
    pub async fn post(&self, parameters: PostParameters) -> Result<PostResponseItem, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
