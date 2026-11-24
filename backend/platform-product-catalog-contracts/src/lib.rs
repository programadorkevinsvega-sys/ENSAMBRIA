//! Public PLATFORM product catalog contract shared by in-process and Tonic adapters.
use std::{future::Future, pin::Pin};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

pub type ContractFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductCatalogEntry {
    pub product_id: Uuid,
    pub product_key: String,
    pub display_name: String,
    pub brand_name: String,
    pub configuration_version: u32,
    pub configuration_schema_version: String,
    pub configuration: Value,
    pub domains: Vec<ProductDomain>,
    pub markets: Vec<ProductMarket>,
    pub surfaces: Vec<ProductSurface>,
    pub capabilities: Vec<ProductCapability>,
    pub published_at_unix: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProductDomain {
    pub domain_name: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProductMarket {
    pub country_code: String,
    pub currency_code: String,
    pub locale: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProductSurface {
    pub surface_code: String,
    pub configuration_version: u32,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProductCapability {
    pub capability_code: String,
    pub contract_version: String,
    pub implementation_status: String,
    pub product_status: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductConfiguration {
    pub product_id: Uuid,
    pub version: u32,
    pub schema_version: String,
    pub canonical_json: Value,
    pub published_at_unix: Option<i64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProductCatalogFailure {
    InvalidRequest,
    NotFound,
    Unavailable,
}

pub trait ProductCatalog: Send + Sync {
    fn get_active_product<'a>(
        &'a self,
        product_id: Uuid,
    ) -> ContractFuture<'a, Result<ProductCatalogEntry, ProductCatalogFailure>>;

    fn resolve_active_domain<'a>(
        &'a self,
        domain_name: &'a str,
    ) -> ContractFuture<'a, Result<Option<ProductCatalogEntry>, ProductCatalogFailure>>;

    fn get_published_configuration<'a>(
        &'a self,
        product_id: Uuid,
        version: u32,
    ) -> ContractFuture<'a, Result<Option<ProductConfiguration>, ProductCatalogFailure>>;
}
