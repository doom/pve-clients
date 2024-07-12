pub mod acl;
pub mod domains;
pub mod groups;
pub mod openid;
pub mod password;
pub mod permissions;
pub mod roles;
pub mod tfa;
pub mod ticket;
pub mod users;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub subdir: String,
}

#[derive(Debug, Clone)]
pub struct AccessClient<T> {
    client: T,
    path: String,
}

impl<T> AccessClient<T>
where
    T: Clone,
{
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "access".to_string(),
        }
    }

    pub fn users(&self) -> users::UsersClient<T> {
        users::UsersClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn groups(&self) -> groups::GroupsClient<T> {
        groups::GroupsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn roles(&self) -> roles::RolesClient<T> {
        roles::RolesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn acl(&self) -> acl::AclClient<T> {
        acl::AclClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn domains(&self) -> domains::DomainsClient<T> {
        domains::DomainsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn openid(&self) -> openid::OpenidClient<T> {
        openid::OpenidClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn tfa(&self) -> tfa::TfaClient<T> {
        tfa::TfaClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn ticket(&self) -> ticket::TicketClient<T> {
        ticket::TicketClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn password(&self) -> password::PasswordClient<T> {
        password::PasswordClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn permissions(&self) -> permissions::PermissionsClient<T> {
        permissions::PermissionsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AccessClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Directory index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncAccessClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncAccessClient<T>
where
    T: Clone,
{
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "access".to_string(),
        }
    }

    pub fn users(&self) -> users::AsyncUsersClient<T> {
        users::AsyncUsersClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn groups(&self) -> groups::AsyncGroupsClient<T> {
        groups::AsyncGroupsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn roles(&self) -> roles::AsyncRolesClient<T> {
        roles::AsyncRolesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn acl(&self) -> acl::AsyncAclClient<T> {
        acl::AsyncAclClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn domains(&self) -> domains::AsyncDomainsClient<T> {
        domains::AsyncDomainsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn openid(&self) -> openid::AsyncOpenidClient<T> {
        openid::AsyncOpenidClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn tfa(&self) -> tfa::AsyncTfaClient<T> {
        tfa::AsyncTfaClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn ticket(&self) -> ticket::AsyncTicketClient<T> {
        ticket::AsyncTicketClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn password(&self) -> password::AsyncPasswordClient<T> {
        password::AsyncPasswordClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn permissions(&self) -> permissions::AsyncPermissionsClient<T> {
        permissions::AsyncPermissionsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncAccessClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Directory index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
