use std::collections::HashSet;

use async_graphql::Context;
use cached::proc_macro::once;
use ecow::EcoString;
use jsonwebtoken::{jwk::JwkSet, Algorithm, DecodingKey, Validation};
use reqwest::Url;

#[derive(Clone, Copy)]
pub enum Scope {
    ReadResource,
    WriteResource,
    Challenge,
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::fmt::Debug for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Scope {
    pub fn as_str(&self) -> &str {
        match self {
            Scope::ReadResource => "read:resource",
            Scope::WriteResource => "write:resource",
            Scope::Challenge => "challenge",
        }
    }
}

#[derive(Clone)]
pub struct AuthBuilder {
    pub logto_domain: EcoString,
    pub resource_indicator: EcoString,
}

impl AuthBuilder {
    pub async fn build(&self, jwt: &str) -> Result<Auth, AuthError> {
        #[derive(Debug, serde::Deserialize)]
        struct Claim {
            scope: String,
        }

        let header = jsonwebtoken::decode_header(&jwt).map_err(AuthError::DecodeJwtHeader)?;
        let key = get_jwk_decoding_key(&self.logto_domain).await?;
        let validation =
            get_validation_parameter(&self.logto_domain, &self.resource_indicator, header.alg);

        let token_data =
            jsonwebtoken::decode::<Claim>(jwt, &key, &validation).map_err(AuthError::DecodeJwt)?;
        let scopes = token_data
            .claims
            .scope
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();

        Ok(Auth { scopes })
    }
}

pub struct Auth {
    scopes: HashSet<String>,
}

impl Auth {
    pub fn has_scope(&self, scope: Scope) -> bool {
        self.scopes.contains(scope.as_str())
    }
}

pub trait ContextAuthExt {
    fn require_scope(&self, scope: Scope) -> Result<(), super::error::Error>;
}

impl ContextAuthExt for Context<'_> {
    fn require_scope(&self, scope: Scope) -> Result<(), super::error::Error> {
        let Ok(auth) = self.data::<Auth>() else {
            return Err(super::error::Error {
                code: super::error::ErrorCode::Unauthorized,
                title: EcoString::inline("Unauthorized"),
                details: "You must provide a credential to access this API.".into(),
                error: None,
            });
        };

        if !auth.has_scope(scope) {
            return Err(super::error::Error {
                code: super::error::ErrorCode::Unauthorized,
                title: EcoString::inline("Unauthorized"),
                details: format!("{scope} is required to perform this action").into(),
                error: None,
            });
        }

        Ok(())
    }
}

fn get_logto_endpoint(logto_domain: &str, endpoint: &str) -> reqwest::Url {
    Url::parse(logto_domain)
        .expect("logto_domain must be a valid URL")
        .join(endpoint)
        .expect("endpoint must be a valid path")
}

/// Get the JWKSet from the OIDC server.
#[once(time = 3600, sync_writes = true, result = true)]
async fn get_jwkset(logto_domain: &str) -> Result<JwkSet, AuthError> {
    let jwkset = reqwest::get(get_logto_endpoint(logto_domain, "oidc/jwks"))
        .await
        .map_err(AuthError::GetJwtSetFailed)?;
    Ok(jwkset.json().await.map_err(AuthError::GetJwtSetFailed)?)
}

// Get a JWK from the JWKSet.
async fn get_jwk_decoding_key(logto_domain: &str) -> Result<DecodingKey, AuthError> {
    let jwkset = get_jwkset(logto_domain).await?;

    let decoding_key = jwkset
        .keys
        .into_iter()
        .find_map(|key| DecodingKey::from_jwk(&key).ok());

    decoding_key.ok_or(AuthError::MissingJwkInOidc)
}

fn get_validation_parameter(
    logto_domain: &str,
    resource_indicator: &str,
    alg: Algorithm,
) -> Validation {
    let mut validation = Validation::new(alg);
    validation.validate_nbf = true;
    validation.set_issuer(&[get_logto_endpoint(logto_domain, "oidc")]);
    validation.set_audience(&[resource_indicator]);

    validation
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("get JWT set: {0}")]
    GetJwtSetFailed(reqwest::Error),

    #[error("missing JWK in OIDC")]
    MissingJwkInOidc,

    #[error("decode JWT header: {0}")]
    DecodeJwtHeader(jsonwebtoken::errors::Error),

    #[error("decode JWT: {0}")]
    DecodeJwt(jsonwebtoken::errors::Error),
}
