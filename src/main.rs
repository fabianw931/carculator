use google_sheets4::Sheets;

mod auth;
mod config;
mod http_client;
mod sheets;

#[tokio::main]
async fn main() {
    let config = config::Config::new();
    let client = http_client::http_client();
    let auth = auth::auth(&config, client.clone()).await;

    let mut hub = Sheets::new(client.clone(), auth);
    let result = sheets::read(&hub, &config).await;
}
