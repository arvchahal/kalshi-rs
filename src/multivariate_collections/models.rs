use derive_more::Display;
use serde::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Display, Debug, Clone)]
#[display("AssociatedEvent {{ ticker: {ticker}, is_yes_only: {is_yes_only}, size_min: {size_min}, size_max: {size_max}, active_quoters: {active_quoters:?} }}")]
pub struct AssociatedEvent {
    pub ticker: String,
    pub is_yes_only: bool,
    pub size_max: u64,
    pub size_min: u64,
    pub active_quoters: Vec<String>,
}

#[derive(Deserialize, Display, Debug, Clone)]
#[display("MultivariateContract {{ collection_ticker: {collection_ticker}, title: {title} }}")]
pub struct MultivariateContract {
    pub collection_ticker: String,
    pub series_ticker: String,
    pub title: String,
    pub description: String,
    pub open_date: String,
    pub close_date: String,
    pub associated_events: Vec<AssociatedEvent>,
    pub associated_event_tickers: Vec<String>,
    pub is_ordered: bool,
    pub is_single_market_per_event: bool,
    pub is_all_yes: bool,
    pub size_min: u64,
    pub size_max: u64,
    pub functional_description: String,
}

#[derive(Deserialize, Display, Debug, Clone)]
#[display("GetMultivariateEventCollectionResponse {{ multivariate_contract: {multivariate_contract} }}")]
pub struct GetMultivariateEventCollectionResponse {
    pub multivariate_contract: MultivariateContract,
}

#[derive(Deserialize, Display, Debug, Clone)]
#[display("GetMultivariateEventCollectionsResponse {{ cursor: {cursor:?}, multivariate_contracts: {multivariate_contracts:?} }}")]
pub struct GetMultivariateEventCollectionsResponse {
    pub multivariate_contracts: Vec<MultivariateContract>,
    pub cursor: Option<String>,
}

// pub struct LookupBundleResponse{

// }