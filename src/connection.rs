use bytes::{BufMut, BytesMut};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

const MULTISTREAM_PROTOCOL: &str = "/multistream/1.0.0";
const NOISE_PROTOCOL: &str = "/noise";

struct MultistreamSelectConnection<'a> {
    transport: Framed<&'a mut TcpStream, LengthDelimitedCodec>,
}

impl<'a> MultistreamSelectConnection<'a> {
    fn new(stream: &'a mut TcpStream) -> Self {
        let transport = LengthDelimitedCodec::builder()
            .length_field_type::<u8>()
            .new_framed(stream);

        Self { transport }
    }

    async fn request_protocol(&mut self, protocol: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut message = BytesMut::with_capacity(protocol.len() + 1);
        message.put_slice(protocol.as_bytes());
        message.put_bytes(b'\n', 1);
        self.transport.send(message.freeze()).await?;
        Ok(())
    }

    async fn read_response(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let buffer = self.transport.next().await.ok_or("EOF")??;
        let protocol = String::from_utf8_lossy(&buffer[..]);
        Ok(protocol.trim_end_matches('\n').into())
    }
}

pub async fn request_noise_protocol(
    stream: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = MultistreamSelectConnection::new(stream);
    connection.request_protocol(MULTISTREAM_PROTOCOL).await?;
    connection.request_protocol(NOISE_PROTOCOL).await?;

    connection.read_response().await?;
    let response = connection.read_response().await?;
    if response == NOISE_PROTOCOL {
        println!("1. Noise protocol negotiated successfully.");
    } else {
        println!(
            "⚠️ Failed to negotiate noise protocol. Response: {}",
            response
        );
    }

    Ok(())
}
