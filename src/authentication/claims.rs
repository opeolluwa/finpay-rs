use std::fmt::Display;
use std::time::Duration;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AuthenticationError;
use crate::utils::extract_env;

// Fixed duration constants (corrected from your original)
pub const FIVE_MINUTES: Duration = Duration::from_secs(5 * 60);
pub const TWENTY_FIVE_MINUTES: Duration = Duration::from_secs(25 * 60);
pub const TEN_MINUTES: Duration = Duration::from_secs(10 * 60);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub(crate) iss: String,
    pub(crate) sub: String,
    pub(crate) iat: i64,
    pub(crate) exp: i64,
    pub(crate) email: String,
    pub(crate) aud: String,
    pub(crate) user_identifier: Uuid,
}

impl Default for Claims {
    fn default() -> Self {
        Self {
            iss: "finpay.app".to_string(),
            sub: Default::default(),
            iat: Default::default(),
            exp: Default::default(),
            email: Default::default(),
            aud: "finpay.mobile".to_string(),
            user_identifier: Default::default(),
        }
    }
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "user_identifier: {}\nemail: {}\niss: {}\naud: {}\nexp: {}",
            self.user_identifier, self.email, self.iss, self.aud, self.exp
        )
    }
}

pub struct ClaimsBuilder {
    claims: Claims,
    validity: Option<Duration>,
}

impl ClaimsBuilder {
    pub fn new() -> Self {
        Self {
            claims: Claims::default(),
            validity: None,
        }
    }

    pub fn email(mut self, email: &str) -> Self {
        self.claims.email = email.into();
        self
    }

    pub fn user_identifier(mut self, user_identifier: &Uuid) -> Self {
        self.claims.user_identifier = *user_identifier;
        self
    }

    pub fn subject<T: Into<String>>(mut self, subject: T) -> Self {
        self.claims.sub = subject.into();
        self
    }

    pub fn issuer<T: Into<String>>(mut self, issuer: T) -> Self {
        self.claims.iss = issuer.into();
        self
    }

    pub fn audience<T: Into<String>>(mut self, audience: T) -> Self {
        self.claims.aud = audience.into();
        self
    }

    pub fn validity(mut self, duration: Duration) -> Self {
        self.validity = Some(duration);
        self
    }

    pub fn issued_at(mut self, timestamp: i64) -> Self {
        self.claims.iat = timestamp;
        self
    }

    pub fn expires_at(mut self, timestamp: i64) -> Self {
        self.claims.exp = timestamp;
        self
    }

    pub fn build(mut self) -> Result<Claims, AuthenticationError> {
        if self.claims.email.is_empty() {
            return Err(AuthenticationError::ValidationError(
                "Email is required".to_string(),
            ));
        }

        if self.claims.user_identifier == Uuid::nil() {
            return Err(AuthenticationError::ValidationError(
                "User identifier is required".to_string(),
            ));
        }

        let now = chrono::Utc::now().timestamp();

        // Set issued at time if not already set
        if self.claims.iat == 0 {
            self.claims.iat = now;
        }

        // Set expiration time based on validity or default
        if self.claims.exp == 0 {
            let validity = self.validity.unwrap_or(TWENTY_FIVE_MINUTES);
            self.claims.exp = now + validity.as_secs() as i64;
        }

        // Set subject to user_identifier if not set
        if self.claims.sub.is_empty() {
            self.claims.sub = self.claims.user_identifier.to_string();
        }

        Ok(self.claims)
    }

    /// Convenience method to build claims and generate token in one step
    pub fn build_and_sign(self) -> Result<String, AuthenticationError> {
        let claims = self.build()?;
        claims.generate_token()
    }
}

impl Default for ClaimsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Keys {
    encoding: EncodingKey,
    pub(crate) decoding: DecodingKey,
}

impl Keys {
    pub(crate) fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

impl Claims {
    /// Create new Claims with email and user_identifier
    pub fn new(email: &str, user_identifier: &Uuid) -> Self {
        Self {
            email: email.to_string(),
            user_identifier: user_identifier.to_owned(),
            ..Default::default()
        }
    }

    /// Create a ClaimsBuilder for more flexible construction
    pub fn builder() -> ClaimsBuilder {
        ClaimsBuilder::new()
    }

    /// Generate a JWT token from the claims
    pub fn generate_token(&self) -> Result<String, AuthenticationError> {
        let claim = Claims {
            email: self.email.clone(),
            user_identifier: self.user_identifier,
            iat: self.iat,
            exp: self.exp,
            ..Default::default()
        };

        let secret = extract_env::<String>("JWT_SIGNING_KEY").map_err(AuthenticationError::from)?;
        let encoding_key = Keys::new(secret.as_bytes()).encoding;

        let token =
            encode(&Header::default(), &claim, &encoding_key).map_err(AuthenticationError::from)?;

        Ok(token)
    }

    /// Generate token with custom validity (legacy method for backward compatibility)
    pub fn generate_token_with_validity(
        &self,
        validity: Duration,
    ) -> Result<String, AuthenticationError> {
        let now = chrono::Utc::now().timestamp();
        let claim = Claims {
            email: self.email.clone(),
            user_identifier: self.user_identifier,
            iat: now,
            exp: now + validity.as_secs() as i64,
            iss: self.iss.to_owned(),
            sub: self.sub.to_owned(),
            aud: self.aud.to_owned(),
        };

        let secret = extract_env::<String>("JWT_SIGNING_KEY").map_err(AuthenticationError::from)?;
        let encoding_key = Keys::new(secret.as_bytes()).encoding;

        let token =
            encode(&Header::default(), &claim, &encoding_key).map_err(AuthenticationError::from)?;

        Ok(token)
    }

    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now().timestamp();
        self.exp < now
    }

    /// Get time remaining until expiration
    pub fn time_until_expiry(&self) -> Duration {
        let now = chrono::Utc::now().timestamp();
        let remaining = (self.exp - now).max(0) as u64;
        Duration::from_secs(remaining)
    }

    pub fn from_token(token: &str) -> Result<Self, AuthenticationError> {
        let secret = extract_env::<String>("JWT_SIGNING_KEY").map_err(AuthenticationError::from)?;
        let decoding_key = Keys::new(secret.as_bytes()).decoding;

        let mut jwt_validation = Validation::default();
        jwt_validation.set_audience(&["finpay.mobile"]);

        let token_data =
            decode::<Claims>(token, &decoding_key, &jwt_validation).map_err(|err| {
                log::error!("failed to decode JWT token due to {err}");
                AuthenticationError::InvalidToken
            })?;

        Ok(token_data.claims)
    }
}
