pub mod aliases;
pub mod groups;
pub mod ipset;
pub mod macros;
pub mod options;
pub mod refs;
pub mod rules;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct FirewallClient<T> {
    client: T,
    path: String,
}

impl<T> FirewallClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "firewall"),
        }
    }

    pub fn groups(&self) -> groups::GroupsClient<T> {
        groups::GroupsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rules(&self) -> rules::RulesClient<T> {
        rules::RulesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn ipset(&self) -> ipset::IpsetClient<T> {
        ipset::IpsetClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn aliases(&self) -> aliases::AliasesClient<T> {
        aliases::AliasesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn options(&self) -> options::OptionsClient<T> {
        options::OptionsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn macros(&self) -> macros::MacrosClient<T> {
        macros::MacrosClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn refs(&self) -> refs::RefsClient<T> {
        refs::RefsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> FirewallClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Directory index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncFirewallClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncFirewallClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "firewall"),
        }
    }

    pub fn groups(&self) -> groups::AsyncGroupsClient<T> {
        groups::AsyncGroupsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn rules(&self) -> rules::AsyncRulesClient<T> {
        rules::AsyncRulesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn ipset(&self) -> ipset::AsyncIpsetClient<T> {
        ipset::AsyncIpsetClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn aliases(&self) -> aliases::AsyncAliasesClient<T> {
        aliases::AsyncAliasesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn options(&self) -> options::AsyncOptionsClient<T> {
        options::AsyncOptionsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn macros(&self) -> macros::AsyncMacrosClient<T> {
        macros::AsyncMacrosClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn refs(&self) -> refs::AsyncRefsClient<T> {
        refs::AsyncRefsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncFirewallClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Directory index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
