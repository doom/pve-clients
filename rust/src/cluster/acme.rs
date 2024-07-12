pub mod account;
pub mod challenge_schema;
pub mod directories;
pub mod plugins;
pub mod tos;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct AcmeClient<T> {
    client: T,
    path: String,
}

impl<T> AcmeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "acme"),
        }
    }

    pub fn plugins(&self) -> plugins::PluginsClient<T> {
        plugins::PluginsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn account(&self) -> account::AccountClient<T> {
        account::AccountClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn tos(&self) -> tos::TosClient<T> {
        tos::TosClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn directories(&self) -> directories::DirectoriesClient<T> {
        directories::DirectoriesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn challenge_schema(&self) -> challenge_schema::ChallengeSchemaClient<T> {
        challenge_schema::ChallengeSchemaClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AcmeClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "ACMEAccount index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncAcmeClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncAcmeClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "acme"),
        }
    }

    pub fn plugins(&self) -> plugins::AsyncPluginsClient<T> {
        plugins::AsyncPluginsClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn account(&self) -> account::AsyncAccountClient<T> {
        account::AsyncAccountClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn tos(&self) -> tos::AsyncTosClient<T> {
        tos::AsyncTosClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn directories(&self) -> directories::AsyncDirectoriesClient<T> {
        directories::AsyncDirectoriesClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn challenge_schema(&self) -> challenge_schema::AsyncChallengeSchemaClient<T> {
        challenge_schema::AsyncChallengeSchemaClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncAcmeClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "ACMEAccount index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
