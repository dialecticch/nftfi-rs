# nftfi

client library for [nftfi](https://www.nftfi.com/) api

## Examples
```rust
let client = NFTFiClient::new().expect("client");
let listings = client.get_listings().await.expect("listings");
```

License: MIT
