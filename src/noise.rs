use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use snow::Builder;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

const MSG_LEN: usize = 65535;

pub async fn perform_handshake(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let static_keypair = Builder::new("Noise_XX_25519_ChaChaPoly_SHA256".parse().unwrap())
        .generate_keypair()
        .unwrap();

    let mut initiator = Builder::new("Noise_XX_25519_ChaChaPoly_SHA256".parse().unwrap())
        .local_private_key(&static_keypair.private)
        .build_initiator()
        .unwrap();

    let mut transport = LengthDelimitedCodec::builder()
        .length_field_type::<u16>()
        .max_frame_length(MSG_LEN)
        .new_framed(stream);

    // -> e
    let mut noise_msg = [0u8; MSG_LEN];
    let len = initiator.write_message(&[], &mut noise_msg)?;
    println!("-> e: Sent first handshake message, length: {}", len);
    transport
        .send(Bytes::copy_from_slice(&noise_msg[..len]))
        .await?;

    // <- e, ee, s, es
    let rcv_buf = transport.next().await.ok_or("EOF")??;
    println!(
        "<- e, ee, s, es: Received response, length: {}",
        rcv_buf.len()
    );

    let mut payload = vec![0u8; rcv_buf.len()];
    initiator.read_message(&rcv_buf, &mut payload)?;
    let payload_len = rcv_buf.len();

    // -> s, se
    let mut buf = [0u8; MSG_LEN];
    let len = initiator.write_message(&payload[..payload_len], &mut buf)?;
    println!("-> s, se: Sent final handshake message, length: {}", len);
    transport.send(Bytes::copy_from_slice(&buf[..len])).await?;

    println!("2. Noise handshake completed successfully.");

    Ok(())
}
