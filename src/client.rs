use crate::{
    errors::NFTFiError,
    models::{Asset, Listing, Listings, Loans, Offers, Projects},
};
use async_stream::try_stream;
use ethereum_types::Address;
use futures::Stream;
use governor::{Quota, RateLimiter};
use reqwest::{Client, ClientBuilder, Method, Request, Url};
use serde::Deserialize;
use std::num::NonZeroU32;

const BASE_URL: &str = "https://api.nftfi.com";

pub struct NFTFiClient {
    client: Client,
}

impl NFTFiClient {
    fn build_req(method: Method, path: &str) -> Result<Request, NFTFiError> {
        let mut url = Url::parse(BASE_URL)?;
        url.set_path(path);
        Ok(Request::new(method, url))
    }

    async fn request<'a, R: ?Sized>(&self, request: Request) -> Result<R, NFTFiError>
    where
        for<'de> R: Deserialize<'de> + 'a,
    {
        tracing::info!("Requesting {}", request.url());
        let text = self
            .client
            .execute(request)
            .await?
            .error_for_status()?
            .text()
            .await?;
        let jd = &mut serde_json::Deserializer::from_str(&text);
        Ok(serde_path_to_error::deserialize(jd)?)
    }

    // Construct new client
    pub fn new() -> Result<Self, NFTFiError> {
        let client = ClientBuilder::new().build()?;
        Ok(Self { client })
    }

    // List all projects
    pub async fn get_projects(&self) -> Result<Projects, NFTFiError> {
        let req = Self::build_req(Method::GET, "/projects")?;
        self.request(req).await
    }

    // List all loan listings
    pub async fn get_listings(&self) -> Result<Listings, NFTFiError> {
        let req = Self::build_req(Method::GET, "/listings")?;
        self.request(req).await
    }

    // Get asset details
    pub async fn get_asset(&self, collection: &Address, id: &str) -> Result<Asset, NFTFiError> {
        let req = Self::build_req(Method::GET, &format!("/asset/0x{collection:02x}/{id}"))?;
        self.request(req).await
    }

    // Get loan offers for an asset
    pub async fn get_offers(&self, collection: &Address, id: &str) -> Result<Offers, NFTFiError> {
        let req = Self::build_req(Method::GET, &format!("/offers/0x{collection:02x}/{id}"))?;
        self.request(req).await
    }

    // Get active and historical loans for an asset
    pub async fn get_loans(&self, collection: &Address, id: &str) -> Result<Loans, NFTFiError> {
        let req = Self::build_req(
            Method::GET,
            &format!("/loans/asset/0x{collection:02x}/{id}"),
        )?;
        self.request(req).await
    }

    // Poll API for new listings
    pub async fn watch_listings(
        &self,
        refresh_seconds: u32,
    ) -> impl Stream<Item = Result<Listing, NFTFiError>> + '_ {
        try_stream! {
            let quota = Quota::per_second(NonZeroU32::new(refresh_seconds).unwrap());
            let rate_limiter = RateLimiter::direct(quota);
            let mut last_date = None;
            loop {
                let mut results = self.get_listings().await?;
                results.sort_by_key(|l| l.listed_date);
                for result in results.into_iter() {
                    match last_date {
                        Some(last) if result.listed_date <= last => {},
                        _ => {
                            last_date = Some(result.listed_date);
                            yield result;
                        }
                    }
                }
                rate_limiter.until_ready().await;
            }
        }
    }
}
