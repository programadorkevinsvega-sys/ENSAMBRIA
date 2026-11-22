use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
    response::IntoResponse,
    routing::{get, post},
};
use ensambria_core::identity_authentication::service::{
    AuthSession, AuthenticationError, AuthenticationService, EmailDelivery, LoginOutcome,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::service::{BusinessError, BusinessService};

pub struct ApiState<D: EmailDelivery> {
    pub auth: Arc<AuthenticationService<D>>,
    pub business: BusinessService,
}

pub fn router<D: EmailDelivery + 'static>(state: Arc<ApiState<D>>) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/v1/auth/register", post(register::<D>))
        .route("/v1/auth/login", post(login::<D>))
        .route("/v1/auth/verify-email", post(verify_email::<D>))
        .route("/v1/auth/recovery", post(recovery::<D>))
        .route("/v1/auth/recovery/complete", post(recovery_complete::<D>))
        .route("/v1/auth/refresh", post(refresh::<D>))
        .route("/v1/auth/mfa/challenge", post(mfa_challenge::<D>))
        .route("/v1/business/establishments", get(list_establishments::<D>).post(create_establishment::<D>))
        .with_state(state)
}

#[derive(Deserialize)]
struct RegisterRequest { email: String, display_name: Option<String>, password: String }
#[derive(Deserialize)]
struct LoginRequest { email: String, password: String }
#[derive(Deserialize)]
struct TokenRequest { token: String }
#[derive(Deserialize)]
struct RecoveryRequest { email: String }
#[derive(Deserialize)]
struct RecoveryCompleteRequest { token: String, new_password: String }
#[derive(Deserialize)]
struct RefreshRequest { refresh_token: String }
#[derive(Deserialize)]
struct MfaRequest { challenge_token: String, factor: String }
#[derive(Deserialize)]
struct CreateEstablishmentRequest { organization_id: Uuid, branch_id: Option<Uuid>, display_name: String }

#[derive(Serialize)]
struct MessageResponse { status: &'static str }
#[derive(Serialize)]
struct AuthResponse {
    status: &'static str,
    identity_id: Option<Uuid>,
    session_id: Option<Uuid>,
    access_token: Option<String>,
    refresh_token: Option<String>,
    access_expires_at: Option<u64>,
    refresh_expires_at: Option<u64>,
    challenge_token: Option<String>,
}

async fn register<D: EmailDelivery>(State(state): State<Arc<ApiState<D>>>, Json(request): Json<RegisterRequest>) -> impl IntoResponse {
    match state.auth.register(&request.email, request.display_name.as_deref(), &request.password).await {
        Ok(()) => (StatusCode::ACCEPTED, Json(MessageResponse { status: "accepted" })).into_response(),
        Err(error) => auth_error(error),
    }
}
async fn login<D: EmailDelivery>(State(state): State<Arc<ApiState<D>>>, Json(request): Json<LoginRequest>) -> impl IntoResponse {
    match state.auth.login(&request.email, &request.password).await {
        Ok(LoginOutcome::Authenticated(session)) => (StatusCode::OK, Json(session_response(session))).into_response(),
        Ok(LoginOutcome::MfaRequired { challenge_token }) => (StatusCode::ACCEPTED, Json(AuthResponse {
            status: "mfa_required", identity_id: None, session_id: None, access_token: None,
            refresh_token: None, access_expires_at: None, refresh_expires_at: None, challenge_token: Some(challenge_token)
        })).into_response(),
        Err(error) => auth_error(error),
    }
}
async fn verify_email<D: EmailDelivery>(State(state): State<Arc<ApiState<D>>>, Json(request): Json<TokenRequest>) -> impl IntoResponse {
    match state.auth.verify_email(&request.token).await {
        Ok(identity_id) => (StatusCode::OK, Json(serde_json::json!({"status":"verified","identity_id":identity_id}))).into_response(),
        Err(error) => auth_error(error),
    }
}
async fn recovery<D: EmailDelivery>(State(state): State<Arc<ApiState<D>>>, Json(request): Json<RecoveryRequest>) -> impl IntoResponse {
    match state.auth.request_recovery(&request.email).await {
        Ok(()) => (StatusCode::ACCEPTED, Json(MessageResponse { status: "accepted" })).into_response(),
        Err(error) => auth_error(error),
    }
}
async fn recovery_complete<D: EmailDelivery>(State(state): State<Arc<ApiState<D>>>, Json(request): Json<RecoveryCompleteRequest>) -> impl IntoResponse {
    match state.auth.complete_recovery(&request.token, &request.new_password).await {
        Ok(identity_id) => (StatusCode::OK, Json(serde_json::json!({"status":"recovered","identity_id":identity_id}))).into_response(),
        Err(error) => auth_error(error),
    }
}
async fn refresh<D: EmailDelivery>(State(state): State<Arc<ApiState<D>>>, Json(request): Json<RefreshRequest>) -> impl IntoResponse {
    match state.auth.refresh_session(&request.refresh_token).await {
        Ok(session) => (StatusCode::OK, Json(session_response(session))).into_response(),
        Err(error) => auth_error(error),
    }
}
async fn mfa_challenge<D: EmailDelivery>(State(state): State<Arc<ApiState<D>>>, Json(request): Json<MfaRequest>) -> impl IntoResponse {
    match state.auth.verify_mfa_challenge(&request.challenge_token, &request.factor).await {
        Ok(session) => (StatusCode::OK, Json(session_response(session))).into_response(),
        Err(error) => auth_error(error),
    }
}

async fn list_establishments<D: EmailDelivery>(
    State(state): State<Arc<ApiState<D>>>, headers: HeaderMap,
) -> impl IntoResponse {
    let (identity, tenant) = match authenticate_request(&state, &headers).await {
        Ok(value) => value, Err(response) => return response,
    };
    match state.business.list_establishments(identity, tenant).await {
        Ok(items) => (StatusCode::OK, Json(items)).into_response(),
        Err(error) => business_error(error),
    }
}
async fn create_establishment<D: EmailDelivery>(
    State(state): State<Arc<ApiState<D>>>, headers: HeaderMap, Json(request): Json<CreateEstablishmentRequest>,
) -> impl IntoResponse {
    let (identity, tenant) = match authenticate_request(&state, &headers).await {
        Ok(value) => value, Err(response) => return response,
    };
    match state.business.create_establishment(identity, tenant, request.organization_id, request.branch_id, request.display_name).await {
        Ok(item) => (StatusCode::CREATED, Json(item)).into_response(),
        Err(error) => business_error(error),
    }
}

async fn authenticate_request<D: EmailDelivery>(state: &ApiState<D>, headers: &HeaderMap) -> Result<(Uuid, Uuid), axum::response::Response> {
    let token = headers.get(AUTHORIZATION).and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer ")).filter(|v| !v.is_empty())
        .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?;
    let tenant = headers.get("x-tenant-id").and_then(|v| v.to_str().ok())
        .and_then(|v| Uuid::parse_str(v).ok())
        .ok_or_else(|| StatusCode::BAD_REQUEST.into_response())?;
    let identity = state.auth.authenticate_access_token(token).await
        .map_err(|_| StatusCode::UNAUTHORIZED.into_response())?;
    Ok((identity, tenant))
}

fn session_response(session: AuthSession) -> AuthResponse {
    AuthResponse {
        status: "authenticated",
        identity_id: Some(session.identity_id),
        session_id: Some(session.session_id),
        access_token: Some(session.access_token),
        refresh_token: Some(session.refresh_token),
        access_expires_at: session.access_expires_at.duration_since(std::time::UNIX_EPOCH).ok().map(|d| d.as_secs()),
        refresh_expires_at: session.refresh_expires_at.duration_since(std::time::UNIX_EPOCH).ok().map(|d| d.as_secs()),
        challenge_token: None,
    }
}
fn auth_error(error: AuthenticationError) -> axum::response::Response {
    let status = match error {
        AuthenticationError::InvalidInput => StatusCode::BAD_REQUEST,
        AuthenticationError::RateLimited | AuthenticationError::TemporarilyLocked => StatusCode::TOO_MANY_REQUESTS,
        AuthenticationError::InvalidCredentials | AuthenticationError::InvalidAccessToken |
        AuthenticationError::InvalidRefreshToken | AuthenticationError::InvalidRecoveryToken |
        AuthenticationError::InvalidVerificationToken | AuthenticationError::InvalidChallenge |
        AuthenticationError::InvalidMfaFactor => StatusCode::UNAUTHORIZED,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    (status, Json(serde_json::json!({"error":error.to_string()}))).into_response()
}
fn business_error(error: BusinessError) -> axum::response::Response {
    let status = match error {
        BusinessError::Forbidden => StatusCode::FORBIDDEN,
        BusinessError::InvalidInput => StatusCode::BAD_REQUEST,
        BusinessError::OrganizationUnavailable => StatusCode::NOT_FOUND,
        BusinessError::Storage => StatusCode::INTERNAL_SERVER_ERROR,
    };
    (status, Json(serde_json::json!({"error":error.to_string()}))).into_response()
}
