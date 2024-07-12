#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostParameters {
    #[doc = "The storage where the template will be stored"]
    pub storage: String,
    #[doc = "The template which will downloaded"]
    pub template: String,
}

#[derive(Debug, Clone)]
pub struct AplinfoClient<T> {
    client: T,
    path: String,
}

impl<T> AplinfoClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "aplinfo"),
        }
    }
}
impl<T> AplinfoClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Get list of appliances."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }

    #[doc = "Download appliance templates."]
    pub fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters)
    }
}
#[derive(Debug, Clone)]
pub struct AsyncAplinfoClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncAplinfoClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "aplinfo"),
        }
    }
}
impl<T> AsyncAplinfoClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Get list of appliances."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }

    #[doc = "Download appliance templates."]
    pub async fn post(&self, parameters: PostParameters) -> Result<String, T::Error> {
        self.client.post(&self.path, &parameters).await
    }
}
