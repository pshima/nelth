use hyper::Client;
use hyper_tls::HttpsConnector;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder()
    .build::<_, hyper::Body>(https);
    let uri = "https://google.com".parse()?;
    let resp = client.get(uri).await?;
    println!("Response: {}", resp.status());
    println!("Headers: {:#?}\n", resp.headers());
    Ok(())
}
