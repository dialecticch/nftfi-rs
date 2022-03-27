use crate::utils::empty_string_as_none;
use chrono::{DateTime, NaiveDateTime, Utc};
use ethereum_types::{Address, U256};
use serde::{Deserialize, Serialize};

pub type Loans = Vec<Loan>;
pub type Offers = Vec<Offer>;
pub type Listings = Vec<Listing>;
pub type Projects = Vec<Project>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Loan {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "contractName")]
    pub contract_name: ContractName,
    #[serde(rename = "loanId")]
    pub loan_id: i64,
    #[serde(rename = "__v")]
    pub v: i64,
    #[serde(rename = "assetCategory")]
    pub asset_category: String,
    #[serde(rename = "assetName")]
    pub asset_name: String,
    pub borrower: Address,
    #[serde(rename = "interestIsProRated")]
    pub interest_is_pro_rated: bool,
    pub lender: Address,
    pub liquidated: bool,
    #[serde(rename = "loanDueTime")]
    pub loan_due_time: DateTime<Utc>,
    #[serde(rename = "loanDuration")]
    pub loan_duration: i64,
    #[serde(rename = "loanERC20Denomination")]
    pub loan_erc20_denomination: Option<Address>,
    #[serde(rename = "loanInterestRateForDurationInBasisPoints")]
    pub loan_interest_rate_for_duration_in_basis_points: i64,
    #[serde(rename = "loanPrincipalAmount")]
    pub loan_principal_amount: f64,
    #[serde(rename = "loanStartTime")]
    pub loan_start_time: DateTime<Utc>,
    #[serde(rename = "maximumRepaymentAmount")]
    pub maximum_repayment_amount: f64,
    #[serde(rename = "nftCollateralContract")]
    pub nft_collateral_contract: Address,
    #[serde(rename = "nftCollateralId")]
    pub nft_collateral_id: String,
    #[serde(rename = "nftKey")]
    pub nft_key: String,
    pub repaid: bool,
    #[serde(rename = "activeTransactionHash")]
    pub active_transaction_hash: Option<serde_json::Value>,
    #[serde(rename = "activeTransactionName")]
    pub active_transaction_name: Option<serde_json::Value>,
    #[serde(rename = "borrowerNotifiedLoanIsDue")]
    pub borrower_notified_loan_is_due: bool,
    #[serde(rename = "adminFee")]
    pub admin_fee: U256,
    #[serde(rename = "amountPaidToLender")]
    pub amount_paid_to_lender: U256,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Offer {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    pub lender: Address,
    #[serde(rename = "nftCollateralContract")]
    pub nft_collateral_contract: Address,
    #[serde(rename = "nftCollateralId")]
    pub nft_collateral_id: String,
    #[serde(rename = "__v")]
    pub v: i64,
    #[serde(rename = "adminFeeInBasisPoints")]
    pub admin_fee_in_basis_points: i64,
    #[serde(rename = "assetCategory")]
    pub asset_category: String,
    #[serde(rename = "assetName")]
    pub asset_name: String,
    pub borrower: Address,
    #[serde(rename = "contractName")]
    pub contract_name: ContractName,
    #[serde(rename = "interestIsProRated")]
    pub interest_is_pro_rated: bool,
    #[serde(rename = "lenderNonce")]
    pub lender_nonce: String,
    #[serde(rename = "loanDuration")]
    pub loan_duration: i64,
    #[serde(rename = "loanERC20Denomination")]
    pub loan_erc20_denomination: Option<Address>,
    #[serde(rename = "loanInterestRateForDurationInBasisPoints")]
    pub loan_interest_rate_for_duration_in_basis_points: U256,
    #[serde(rename = "loanPrincipalAmount")]
    pub loan_principal_amount: f64,
    #[serde(rename = "maximumRepaymentAmount")]
    pub maximum_repayment_amount: f64,
    #[serde(rename = "nftKey")]
    pub nft_key: String,
    #[serde(rename = "offerDate")]
    pub offer_date: DateTime<Utc>,
    #[serde(rename = "signedMessage")]
    pub signed_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    #[serde(rename = "nftCollateralId")]
    pub nft_collateral_id: String,
    #[serde(rename = "nftCollateralContract")]
    pub nft_collateral_contract: Address,
    pub name: String,
    #[serde(rename = "projectName")]
    pub project_name: String,
    pub description: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    // #[serde(rename = "animationUrl")]
    // pub animation_url: Option<serde_json::Value>,
    pub traits: Vec<AssetTrait>,
    #[serde(rename = "dataUri")]
    pub data_uri: String,
    #[serde(rename = "supportedInterface")]
    pub supported_interface: String,
    pub owner: Owner,
    pub whitelisted: bool,
    pub status: Status,
    // #[serde(rename = "desiredLoanDuration")]
    // pub desired_loan_duration: Option<serde_json::Value>,
    #[serde(rename = "desiredLoanPrincipalAmount")]
    pub desired_loan_principal_amount: U256,
    #[serde(
        deserialize_with = "empty_string_as_none",
        rename = "desiredLoanCurrency"
    )]
    pub desired_loan_currency: Option<Address>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub address: Address,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetTrait {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "nftCollateralContract")]
    pub nft_collateral_contract: Address,
    #[serde(rename = "nftCollateralId")]
    pub nft_collateral_id: String,
    #[serde(rename = "__v")]
    pub v: i64,
    pub borrower: String,
    #[serde(rename = "contractName")]
    pub contract_name: ContractName,
    #[serde(
        deserialize_with = "empty_string_as_none",
        rename = "desiredLoanCurrency"
    )]
    pub desired_loan_currency: Option<Address>,
    #[serde(rename = "desiredLoanDuration")]
    pub desired_loan_duration: Option<U256>,
    #[serde(rename = "desiredLoanPrincipalAmount")]
    pub desired_loan_principal_amount: Option<U256>,
    #[serde(rename = "listedDate")]
    pub listed_date: DateTime<Utc>,
    #[serde(rename = "nftKey")]
    pub nft_key: String,
    pub nonce: String,
    #[serde(rename = "signedMessage")]
    pub signed_message: String,
    pub status: Status,
    pub whitelisted: bool,
    pub name: Option<String>,
    #[serde(rename = "projectName")]
    pub project_name: String,
    pub description: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    #[serde(rename = "animationUrl")]
    pub animation_url: Option<String>,
    pub traits: Vec<Trait>,
    #[serde(rename = "dataUri")]
    pub data_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trait {
    pub trait_type: Option<String>,
    pub value: Value,
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub display_type: Option<String>,
    pub max_value: Option<String>,
    pub trait_count: Option<i64>,
    // pub order: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "_id")]
    pub id: String,
    pub address: String,
    #[serde(rename = "__v")]
    pub v: Option<i64>,
    pub description: String,
    #[serde(rename = "disableAnimation")]
    pub disable_animation: bool,
    #[serde(rename = "imgSrc")]
    pub img_src: String,
    #[serde(rename = "isWhitelisted")]
    pub is_whitelisted: bool,
    pub metadata: Metadata,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "contractAddress")]
    pub contract_address: Address,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "imgSrc")]
    pub img_src: Option<String>,
    pub collection: Collection,
    pub address: Address,
    pub asset_contract_type: AssetContractType,
    pub created_date: NaiveDateTime,
    pub nft_version: Option<NftVersion>,
    pub opensea_version: Option<String>,
    pub owner: Option<i64>,
    pub schema_name: SchemaName,
    pub symbol: String,
    pub total_supply: Option<U256>,
    pub external_link: Option<String>,
    pub image_url: Option<String>,
    pub default_to_fiat: bool,
    pub dev_buyer_fee_basis_points: i64,
    pub dev_seller_fee_basis_points: i64,
    pub only_proxied_transfers: bool,
    pub opensea_buyer_fee_basis_points: i64,
    pub opensea_seller_fee_basis_points: i64,
    pub buyer_fee_basis_points: i64,
    pub seller_fee_basis_points: i64,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub payout_address: Option<Address>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    pub banner_image_url: Option<String>,
    pub chat_url: Option<String>,
    pub created_date: NaiveDateTime,
    pub default_to_fiat: bool,
    pub description: Option<String>,
    pub dev_buyer_fee_basis_points: U256,
    pub dev_seller_fee_basis_points: U256,
    pub discord_url: Option<String>,
    pub display_data: DisplayData,
    pub external_url: Option<String>,
    pub featured: bool,
    pub featured_image_url: Option<String>,
    pub hidden: bool,
    pub safelist_request_status: SafelistRequestStatus,
    pub image_url: Option<String>,
    pub is_subject_to_whitelist: bool,
    pub large_image_url: Option<String>,
    pub medium_username: Option<String>,
    pub name: String,
    pub only_proxied_transfers: bool,
    pub opensea_buyer_fee_basis_points: U256,
    pub opensea_seller_fee_basis_points: U256,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub payout_address: Option<Address>,
    pub require_email: bool,
    pub short_description: Option<String>,
    pub slug: String,
    pub telegram_url: Option<String>,
    pub twitter_username: Option<String>,
    pub instagram_username: Option<String>,
    pub wiki_url: Option<String>,
    pub is_nsfw: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayData {
    pub card_display_style: CardDisplayStyle,
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Integer(i64),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContractName {
    #[serde(rename = "v1.loan.fixed")]
    V1LoanFixed,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "listed")]
    Listed,
    #[serde(rename = "repaid")]
    Repaid,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AssetContractType {
    #[serde(rename = "non-fungible")]
    NonFungible,
    #[serde(rename = "semi-fungible")]
    SemiFungible,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CardDisplayStyle {
    #[serde(rename = "contain")]
    Contain,
    #[serde(rename = "cover")]
    Cover,
    #[serde(rename = "padded")]
    Padded,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SafelistRequestStatus {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "not_requested")]
    NotRequested,
    #[serde(rename = "verified")]
    Verified,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NftVersion {
    #[serde(rename = "1.0")]
    The10,
    #[serde(rename = "3.0")]
    The30,
    #[serde(rename = "unsupported")]
    Unsupported,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SchemaName {
    #[serde(rename = "ERC1155")]
    Erc1155,
    #[serde(rename = "ERC721")]
    Erc721,
}
