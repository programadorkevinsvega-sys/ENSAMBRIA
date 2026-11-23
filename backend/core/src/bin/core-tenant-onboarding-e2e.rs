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
    // Fresh IDs make every manual staging execution independently repeatable.
    let tenant_id = Uuid::now_v7();
    let organization_id = Uuid::now_v7();
    let product_id = Uuid::parse_str(
        &env::var("ENSAMBRIA_E2E_PRODUCT_ID")
            .unwrap_or_else(|_| "018f0f2a-7b3c-7abc-8def-0123456789ab".to_owned()),
    )?;
    let event_id = create_tenant_with_outbox(
        &pool,
        &CreateTenant {
            tenant_id,
            organization_id,
            product_id,
            validated_product_config_version: 1,
            organization_name: format!("ENSAMBRIA staging E2E {tenant_id}"),
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
    println!("PRODUCT_ID={product_id}");
    Ok(())
}
