[![Workflow Status](https://github.com/dialecticch/nftfi-rs/workflows/main/badge.svg)](https://github.com/dialecticch/nftfi-rs/actions?query=workflow%3A%22main%22)

# nftfi

client library for [nftfi](https://www.nftfi.com/) api

## Examples
```rust
use nftfi::client::NFTFiClient;

#[tokio::main]
async fn main() {
   let client = NFTFiClient::new().expect("client");
   let listings = client.get_listings().await.expect("listings");
}
```

License: MIT
