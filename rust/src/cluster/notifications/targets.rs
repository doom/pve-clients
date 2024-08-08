pub mod name;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Comment"]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Show if this target is disabled"]
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub disable: Option<bool>,
    #[doc = "Name of the target."]
    pub name: String,
    #[doc = "Show if this entry was created by a user or was built-in"]
    pub origin: String,
    #[doc = "Type of the target."]
    pub r#type: String,
}

impl GetResponseItem {
    pub fn new(name: String, origin: String, r#type: String) -> Self {
        Self {
            name,
            origin,
            r#type,
            comment: Default::default(),
            disable: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TargetsClient<T> {
    client: T,
    path: String,
}

impl<T> TargetsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "targets"),
        }
    }

    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> TargetsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Returns a list of all entities that can be used as notification targets."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncTargetsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncTargetsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "targets"),
        }
    }

    pub fn name(&self, name: &str) -> name::AsyncNameClient<T> {
        name::AsyncNameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
impl<T> AsyncTargetsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Returns a list of all entities that can be used as notification targets."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
