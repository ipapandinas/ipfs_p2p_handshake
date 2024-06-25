# P2P Node Handshake

This project implements a P2P node handshake with an IPFS node using the Noise protocol in Rust.

## Requirements

- Rust
- `protoc` compiler to compile `.protobuf` message definitions
- A running P2P node

## Local P2P node

For testing puposes you can use [`ipfs/kubo`](https://github.com/ipfs/kubo) and run it with Docker

    ```sh
        docker run --rm --name ipfs_host -v $ipfs_staging:/export -v $ipfs_data:/data/ipfs -p 4001:4001 -p 4001:4001/udp -p 127.0.0.1:8080:8080 -p 127.0.0.1:5001:5001 ipfs/kubo:latest
    ```

## Building and Running

1. Clone the repository
2. Build the project:

   ```sh
   cargo build
   ```

3. Run the project:

   ```sh
   cargo run
   ```

   OR with _debug_ logs:

   ```sh
   RUST_LOG=debug cargo run
   ```

The program begins by negotiating the Noise protocol via a multistream-select negotiation.

It then performs a Noise XX handshake and displays the initial decrypted message received.
