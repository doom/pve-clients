pub mod gotify;
pub mod sendmail;
pub mod smtp;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GetResponseItem {}

#[derive(Debug, Clone)]
pub struct EndpointsClient<T> {
    client: T,
    path: String,
}

impl<T> EndpointsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "endpoints"),
        }
    }

    pub fn sendmail(&self) -> sendmail::SendmailClient<T> {
        sendmail::SendmailClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn gotify(&self) -> gotify::GotifyClient<T> {
        gotify::GotifyClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn smtp(&self) -> smtp::SmtpClient<T> {
        smtp::SmtpClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> EndpointsClient<T>
where
    T: crate::client::HttpClient,
{
    #[doc = "Index for all available endpoint types."]
    pub fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &())
    }
}
#[derive(Debug, Clone)]
pub struct AsyncEndpointsClient<T> {
    client: T,
    path: String,
}

impl<T> AsyncEndpointsClient<T>
where
    T: Clone,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, "endpoints"),
        }
    }

    pub fn sendmail(&self) -> sendmail::AsyncSendmailClient<T> {
        sendmail::AsyncSendmailClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn gotify(&self) -> gotify::AsyncGotifyClient<T> {
        gotify::AsyncGotifyClient::<T>::new(self.client.clone(), &self.path)
    }

    pub fn smtp(&self) -> smtp::AsyncSmtpClient<T> {
        smtp::AsyncSmtpClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AsyncEndpointsClient<T>
where
    T: crate::client::AsyncHttpClient,
{
    #[doc = "Index for all available endpoint types."]
    pub async fn get(&self) -> Result<Vec<GetResponseItem>, T::Error> {
        self.client.get(&self.path, &()).await
    }
}
