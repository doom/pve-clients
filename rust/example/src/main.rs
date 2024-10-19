mod http;

use pve::version::VersionClient;
use url::Url;

use http::{AuthMethod, ClientBuilder};

fn main() {
    let url = std::env::var("PROXMOX_URL").unwrap();
    let realm = std::env::var("PROXMOX_REALM").unwrap();
    let username = std::env::var("PROXMOX_USERNAME").unwrap();
    let token_name = std::env::var("PROXMOX_TOKEN_NAME").unwrap();
    let token_secret = std::env::var("PROXMOX_TOKEN_SECRET").unwrap();

    let auth = AuthMethod::token(format!("{username}@{realm}"), token_name, token_secret);

    let http_client = ClientBuilder::default()
        .with_base_url(Url::parse(&url).unwrap())
        .with_insecure_tls(true)
        .with_auth_method(auth)
        .build()
        .unwrap();

    let version = VersionClient::new(http_client);
    println!("{:#?}", version.get().unwrap());
}
