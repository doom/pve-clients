#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub id: String,
    #[doc = "Human readable name, falls back to id"]
    pub name: String,
    pub schema: Schema,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Schema {}

#[derive(Debug, Clone)]
pub struct ChallengeSchemaClient<T> {
    client: T,
    path: String,
}

impl<T> ChallengeSchemaClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "challenge-schema"),
        }
    }
}
impl<T> ChallengeSchemaClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get schema of ACME challenge types."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncChallengeSchemaClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncChallengeSchemaClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "challenge-schema"),
        }
    }
}
impl<T> AsyncChallengeSchemaClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get schema of ACME challenge types."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
