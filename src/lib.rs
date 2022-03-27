//! client library for [nftfi](https://www.nftfi.com/) api
//!
//! # Examples
//! ```
//! use nftfi::client::NFTFiClient;
//!
//! #[tokio::main]
//! async fn main() {
//!    let client = NFTFiClient::new().expect("client");
//!    let listings = client.get_listings().await.expect("listings");
//! }
//! ```
pub mod client;
pub mod errors;
pub mod models;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::client::NFTFiClient;
    use chrono::Utc;
    use ethereum_types::Address;
    use futures::{pin_mut, StreamExt};

    #[tokio::test]
    async fn test_get_projects() {
        let client = NFTFiClient::new().expect("client");
        let projects = client.get_projects().await.expect("projects");
        assert!(!projects.is_empty());
    }

    #[tokio::test]
    async fn test_get_listings() {
        let client = NFTFiClient::new().expect("client");
        let listings = client.get_listings().await.expect("projects");
        assert!(!listings.is_empty());
    }

    #[tokio::test]
    async fn test_get_asset() {
        let address: Address = "0x026224a2940bfe258d0dbe947919b62fe321f042"
            .parse()
            .expect("address");
        let client = NFTFiClient::new().expect("client");
        client.get_asset(&address, "1378").await.unwrap();
    }

    #[tokio::test]
    async fn test_get_offers() {
        let address: Address = "0x026224a2940bfe258d0dbe947919b62fe321f042"
            .parse()
            .expect("address");
        let client = NFTFiClient::new().expect("client");
        let offers = client.get_offers(&address, "1378").await.expect("offers");
        assert!(!offers.is_empty());
    }

    #[tokio::test]
    async fn test_get_loans() {
        let address: Address = "0x026224a2940bfe258d0dbe947919b62fe321f042"
            .parse()
            .expect("address");
        let client = NFTFiClient::new().expect("client");
        let loans = client.get_loans(&address, "1378").await.expect("loans");
        assert!(!loans.is_empty());
    }

    #[tokio::test]
    async fn test_listing() {
        let client = NFTFiClient::new().expect("client");
        let stream = client.watch_listings(10).await;
        let start_time = Utc::now();
        pin_mut!(stream);
        while let Some(Ok(listing)) = stream.next().await {
            if listing.listed_date > start_time {
                break;
            }
        }
    }
}
