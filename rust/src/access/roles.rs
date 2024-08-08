pub mod roleid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub privs: Option<String>,
    pub roleid: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "crate::common::deserialize_option_bool_lax",
        serialize_with = "crate::common::serialize_option_bool_as_u64"
    )]
    pub special: Option<bool>,
}

impl GetResponseItem {
    pub fn new(roleid: String) -> Self {
        Self {
            roleid,
            privs: Default::default(),
            special: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub privs: Option<String>,
    pub roleid: String,
}

impl PostParameters {
    pub fn new(roleid: String) -> Self {
        Self {
            roleid,
            privs: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RolesClient<T> {
    client: T,
    path: String,
}

impl<T> RolesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "roles"),
        }
    }

    pub fn roleid(&self, roleid: &str) -> roleid::RoleidClient<T> {
        roleid::RoleidClient::<T>::new(self.client.clone(), &self.path, roleid)
    }
}
impl<T> RolesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Role index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create new role."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncRolesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncRolesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "roles"),
        }
    }

    pub fn roleid(&self, roleid: &str) -> roleid::AsyncRoleidClient<T> {
        roleid::AsyncRoleidClient::<T>::new(self.client.clone(), &self.path, roleid)
    }
}
impl<T> AsyncRolesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Role index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create new role."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
