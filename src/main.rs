mod connection;

use tokio::net::TcpStream;

const ADDRESS: &str = "127.0.0.1:4001";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(ADDRESS).await?;

    // 1 - multistream-select negotiation to negotiate the Noise protocol
    connection::request_noise_protocol(&mut stream).await?;

    Ok(())
}
