use mini_redis::client;

#[tokio::main]
async fn main() -> mini_redis::Result<()>{
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    println!("result: {:?}", result);
    Ok(())
}
