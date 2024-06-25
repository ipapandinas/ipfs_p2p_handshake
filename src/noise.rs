use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use prost::Message;
use snow::{Builder, TransportState};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::{
    error::NoiseError,
    noise_proto::{KeyType, NoiseHandshakePayload, PublicKey},
};

pub const MSG_LEN: usize = 65535;

pub async fn perform_handshake(
    stream: &mut TcpStream,
) -> Result<(TransportState, Framed<&mut TcpStream, LengthDelimitedCodec>), NoiseError> {
    let id_keypair = libp2p::identity::ed25519::Keypair::generate();
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
    let rcv_buf = transport
        .next()
        .await
        .ok_or(NoiseError::HandshakeFailed("EOF".to_string()))??;
    println!(
        "<- e, ee, s, es: Received response, length: {}",
        rcv_buf.len()
    );

    let mut payload = vec![0u8; rcv_buf.len()];
    initiator.read_message(&rcv_buf, &mut payload)?;

    // -> s, se
    let payload = create_noise_payload(&static_keypair, &id_keypair);
    let mut buf = [0u8; MSG_LEN];
    let len = initiator.write_message(&payload, &mut buf)?;
    println!("-> s, se: Sent final handshake message, length: {}", len);
    transport.send(Bytes::copy_from_slice(&buf[..len])).await?;

    if initiator.is_handshake_finished() {
        println!("2. Noise handshake completed successfully.");
    } else {
        println!("⚠️ Failed to establish noise handshake.");
    }

    let initiator = initiator.into_transport_mode()?;

    Ok((initiator, transport))
}

fn create_noise_payload(
    static_keypair: &snow::Keypair,
    id_keypair: &libp2p::identity::ed25519::Keypair,
) -> Vec<u8> {
    let mut payload = NoiseHandshakePayload::default();
    let identity_key = PublicKey {
        r#type: KeyType::Ed25519 as i32,
        data: id_keypair.public().to_bytes().to_vec(),
    };

    payload.identity_key = Some(identity_key.encode_to_vec());

    let identity_sig =
        id_keypair.sign(&[&b"noise-libp2p-static-key:"[..], &static_keypair.public].concat());
    payload.identity_sig = Some(identity_sig);

    let mut bytes = Vec::with_capacity(payload.encoded_len());
    payload.encode(&mut bytes).unwrap();
    bytes
}

pub fn decrypt_message(
    mut initiator: TransportState,
    rcv_buf: &[u8],
) -> Result<String, NoiseError> {
    let mut raw_decrypted_message = vec![0u8; rcv_buf.len()];
    initiator.read_message(rcv_buf, &mut raw_decrypted_message)?;
    let decrypted_message = String::from_utf8_lossy(&raw_decrypted_message[..]);
    Ok(decrypted_message.trim_end_matches('\n').into())
}
