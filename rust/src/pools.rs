pub mod poolid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub poolid: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    pub poolid: String,
}

#[derive(Debug, Clone)]
pub struct PoolsClient<T> {
    client: T,
    path: String,
}

impl<T> PoolsClient<T>
where
    T: Clone,
{
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "pools".to_string(),
        }
    }

    pub fn poolid(&self, poolid: &str) -> poolid::PoolidClient<T> {
        poolid::PoolidClient::<T>::new(self.client.clone(), &self.path, poolid)
    }
}
impl<T> PoolsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Pool index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Create new pool."]
    pub fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncPoolsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncPoolsClient<T>
where
    T: Clone,
{
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "pools".to_string(),
        }
    }

    pub fn poolid(&self, poolid: &str) -> poolid::AsyncPoolidClient<T> {
        poolid::AsyncPoolidClient::<T>::new(self.client.clone(), &self.path, poolid)
    }
}
impl<T> AsyncPoolsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Pool index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Create new pool."]
    pub async fn post(&self, parameters: PostParameters) -> Result<(), T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
