use ensambria_core::tenant_lifecycle::{CreateTenant, create_tenant_with_outbox};
use sqlx::postgres::PgPoolOptions;
use std::{env, error::Error, time::Duration};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&env::var("CORE_TENANT_DATABASE_URL")?)
        .await?;
    let tenant_id = Uuid::parse_str("018f0f2a-7b3c-7abc-8def-0123456789ae")?;
    let organization_id = Uuid::parse_str("018f0f2a-7b3c-7abc-8def-0123456789b1")?;
    let product_id = Uuid::parse_str("018f0f2a-7b3c-7abc-8def-0123456789ab")?;
    let event_id = create_tenant_with_outbox(
        &pool,
        &CreateTenant {
            tenant_id,
            organization_id,
            product_id,
            validated_product_config_version: 1,
            organization_name: "ENSAMBRIA E2E tenant".to_owned(),
            country_code: "CO".to_owned(),
            currency_code: "COP".to_owned(),
            locale: "es-CO".to_owned(),
            time_zone: "America/Bogota".to_owned(),
            correlation_id: Uuid::now_v7(),
        },
    )
    .await?;
    println!("TENANT_ID={tenant_id}");
    println!("EVENT_ID={event_id}");
    Ok(())
}
