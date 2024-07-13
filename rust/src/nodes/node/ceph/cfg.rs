pub mod db;
pub mod raw;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct CfgClient<T> {
    client: T,
    path: String,
}

impl<T> CfgClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "cfg"),
        }
    }

    pub fn raw(&self) -> raw::RawClient<T> {
        raw::RawClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn db(&self) -> db::DbClient<T> {
        db::DbClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CfgClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Directory index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncCfgClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncCfgClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "cfg"),
        }
    }

    pub fn raw(&self) -> raw::AsyncRawClient<T> {
        raw::AsyncRawClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn db(&self) -> db::AsyncDbClient<T> {
        db::AsyncDbClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncCfgClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Directory index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
