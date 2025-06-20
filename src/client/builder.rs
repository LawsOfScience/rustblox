use std::sync::{Arc, RwLock};

use crate::client::RustbloxClient;
use crate::error::ClientError;

/// Builds a Rustblox client.
///
/// # Fields
/// - `reqwest_builder`: A holder for the RustbloxClient's internal
/// reqwest client. Does not ever need to be set by the user.
/// - `roblox_cookie`: A .ROBLOSECURITY cookie to be used to authenticate
/// certain requests. Does not need to be set if the user will not use authenticated
/// endpoints (which are labeled). Can be set by [`insert_cookie`](RustbloxClientBuilder::insert_cookie).
/// - `auto_reauth`: Controls whether the Rustblox client built by this will attempt to
/// automatically refresh its `x-csrf-token`. True by default. Can be manually overridden by
/// [`automatic_reauthentication`](RustbloxClientBuilder::automatic_reauthentication).
pub struct RustbloxClientBuilder {
    reqwest_builder: reqwest::ClientBuilder,
    roblox_cookie: Option<String>,
    auto_reauth: bool,
}

impl Default for RustbloxClientBuilder {
    fn default() -> Self {
        RustbloxClientBuilder::new()
    }
}

#[inline]
fn get_user_agent() -> String {
    format!("Rustblox v{}", env!("CARGO_PKG_VERSION"))
}

impl RustbloxClientBuilder {
    /// Sets whether or not the client should automatically reauthenticate itself if the Roblox
    /// API returns a 403 status code.
    /// The RustbloxClient will only attempt reauthentication once (see the definition of insanity
    /// for why).
    ///
    /// # Errors
    ///
    /// This function cannot error.
    #[inline]
    pub fn automatic_reauthentication(mut self, auto_reauth: bool) -> Self {
        self.auto_reauth = auto_reauth;
        self
    }

    /// Attempts to use a `RustbloxClientBuilder` to construct a `RustbloxClient`. This method will fail if reqwest fails to build a client.
    ///
    /// # Errors
    ///
    /// This function returns an error if the reqwest client builder returns an error.
    pub fn build(self) -> Result<RustbloxClient, ClientError> {
        let built_client = self
            .reqwest_builder
            .build()
            .map_err(|e| ClientError::ReqwestBuildError(e.to_string()))?;

        Ok(RustbloxClient {
            reqwest_client: built_client,
            roblox_cookie: self.roblox_cookie,
            csrf_token: Arc::new(RwLock::new(None)),
            auto_reauth: self.auto_reauth,
        })
    }

    /// Inserts a token into a `RustbloxClientBuilder`.
    ///
    /// # Errors
    ///
    /// This function returns an error if the cookie provided is invalid.
    pub fn insert_cookie(mut self, cookie: &str) -> Result<Self, ClientError> {
        // All .ROBLOSECURITY cookies should start with this
        // unless Roblox just decides to randomly change it some day
        // ...which they might

        (!(cookie.starts_with("_|WARNING")))
            .then_some(ClientError::InvalidCookie)
            .map_or(Ok(()), Err)?;

        let formatted_cookie = format!(".ROBLOSECURITY={cookie}");

        self.roblox_cookie = Some(formatted_cookie);

        Ok(self)
    }

    /// Creates a new `RustbloxClientBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            reqwest_builder: reqwest::ClientBuilder::new().user_agent(get_user_agent()),
            roblox_cookie: None,
            auto_reauth: true,
        }
    }
}
