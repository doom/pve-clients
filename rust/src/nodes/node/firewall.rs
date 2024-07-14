pub mod log;
pub mod options;
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

    pub fn rules(&self) -> rules::RulesClient<T> {
        rules::RulesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn options(&self) -> options::OptionsClient<T> {
        options::OptionsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn log(&self) -> log::LogClient<T> {
        log::LogClient::<T>::new(self.client.clone(), &self.path)
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

    pub fn rules(&self) -> rules::AsyncRulesClient<T> {
        rules::AsyncRulesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn options(&self) -> options::AsyncOptionsClient<T> {
        options::AsyncOptionsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn log(&self) -> log::AsyncLogClient<T> {
        log::AsyncLogClient::<T>::new(self.client.clone(), &self.path)
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
