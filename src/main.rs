use hyper::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let uri = "http://google.com".parse()?;
    let resp = client.get(uri).await?;
    println!("Response: {}", resp.status());
    println!("Headers: {:#?}\n", resp.headers());
    Ok(())
}
