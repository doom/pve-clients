#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    #[doc = "Name of the CRUSH rule."]
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct RulesClient<T> {
    client: T,
    path: String,
}

impl<T> RulesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "rules"),
        }
    }
}
impl<T> RulesClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "List ceph rules."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncRulesClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncRulesClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "rules"),
        }
    }
}
impl<T> AsyncRulesClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "List ceph rules."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
