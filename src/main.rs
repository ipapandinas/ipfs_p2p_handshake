use tokio::net::TcpStream;

const ADDRESS: &str = "127.0.0.1:4001";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut _stream = TcpStream::connect(ADDRESS).await?;

    Ok(())
}
