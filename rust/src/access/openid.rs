pub mod auth_url;
pub mod login;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetResponseItem {
    pub subdir: String,
}

#[derive(Debug, Clone)]
pub struct OpenidClient<T> {
    client: T,
    path: String,
}

impl<T> OpenidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "openid"),
        }
    }

    pub fn auth_url(&self) -> auth_url::AuthUrlClient<T> {
        auth_url::AuthUrlClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn login(&self) -> login::LoginClient<T> {
        login::LoginClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> OpenidClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Directory index."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncOpenidClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncOpenidClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "openid"),
        }
    }

    pub fn auth_url(&self) -> auth_url::AsyncAuthUrlClient<T> {
        auth_url::AsyncAuthUrlClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn login(&self) -> login::AsyncLoginClient<T> {
        login::AsyncLoginClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncOpenidClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Directory index."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
