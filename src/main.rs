mod connection;
mod noise;

pub mod noise_proto {
    include!(concat!(env!("OUT_DIR"), "/ipfs.noise.rs"));
}

use futures_util::stream::StreamExt;
use tokio::net::TcpStream;

const ADDRESS: &str = "127.0.0.1:4001";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(ADDRESS).await?;

    // 1 - multistream-select negotiation to negotiate the Noise protocol
    connection::request_noise_protocol(&mut stream).await?;

    // 2 - noise protocol handshake
    let (initiator, mut transport) = noise::perform_handshake(&mut stream).await?;

    // 3 - ensure there are no pending messages to be read
    if let Some(Ok(rcv_buf)) = transport.next().await {
        println!("Message over secure layer: {:?}", rcv_buf);
        // Decrypt the message
        let decrypted_message = noise::decrypt_message(initiator, &rcv_buf)?;
        println!("Decrypted message: {}", decrypted_message);
    } else {
        println!("No initial message received over secure layer.");
    }

    Ok(())
}
