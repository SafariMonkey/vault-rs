extern crate hashicorp_vault as vault;

#[tokio::main()]
async fn main() {
    let host = "http://localhost:8200";
    let token = "test12345";
    let client = vault::Client::new(host, token).await.unwrap();

    let _ = client.set_secret("foo", "bar");

    let secret = client.get_secret("foo").await.unwrap();

    println!("Secret is \"bar\": {}", secret);
}
