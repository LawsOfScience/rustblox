use crate::client::{RequestComponents, RustbloxClient};
use crate::error::RequestError;
use crate::structs::user::{
    AuthenticatedUserAgeBracket, AuthenticatedUserCountryCode, AuthenticatedUserRoles,
    MinimalAuthenticatedUser,
};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Method;

const BASE_URL: &str = "https://users.roblox.com/v1";

impl RustbloxClient {
    /// **MUST AUTHENTICATE**
    ///
    /// Change the display name to `new_name` for the currently authenticated user.
    /// The `user_id` of the authenticated user must be provided.
    ///
    /// # Errors
    ///
    /// This function will error if:
    /// - You do not have a `.ROBLOSECURITY` cookie set
    /// - The endpoint responds with an error
    ///
    /// Possible error responses include:
    /// - Status 400 code 1: Display name is too short
    /// - Status 400 code 2: Display name is too long
    /// - Status 400 code 3: Display name contains invalid characters
    /// - Status 400 code 4: Display name has been moderated
    /// - Status 401 code 0: Authorization denied
    /// - Status 403 code 0: Token validation failed
    /// - Status 403 code 7: The user ID is invalid
    /// - Status 429 code 5: Display name updates have been throttled
    pub async fn change_display_name(
        &self,
        user_id: usize,
        new_name: String,
    ) -> Result<(), RequestError> {
        let url = format!("{BASE_URL}/users/{user_id}/display-names");

        let body = json!({
            "newDisplayName": new_name,
        });
        let data_size = body.clone().to_string().as_bytes().len();

        let mut headers = HeaderMap::new();
        headers.insert("Content-Length", HeaderValue::from(data_size));
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );

        let components = RequestComponents {
            needs_auth: true,
            method: Method::PATCH,
            url,
            headers: Some(headers),
            body: Some(body.to_string()),
        };

        self.make_request::<serde_json::Value>(components, false)
            .await?;
        Ok(())
    }

    /// **MUST AUTHENTICATE**
    ///
    /// Gets minimal information about the authenticated user.
    ///
    /// # Errors
    ///
    /// This function will error if:
    /// - You do not have a `.ROBLOSECURITY` cookie set
    /// - The endpoint responds with an error
    ///
    /// Possible error responses include:
    /// - Status 401 code 0: Authorization denied
    pub async fn get_authenticated_user(&self) -> Result<MinimalAuthenticatedUser, RequestError> {
        let url = format!("{BASE_URL}/users/authenticated");

        let components = RequestComponents {
            needs_auth: true,
            method: Method::GET,
            url,
            headers: None,
            body: None,
        };

        let data = self
            .make_request::<MinimalAuthenticatedUser>(components, false)
            .await?;
        Ok(data)
    }

    /// **MUST AUTHENTICATE**
    ///
    /// Gets the authenticated user's age bracket
    ///
    /// # Errors
    ///
    /// This function will error if:
    /// - You do not have a `.ROBLOSECURITY` cookie set
    /// - The endpoint responds with an error
    ///
    /// Possible error responses include:
    /// - Status 401 code 0: Authorization denied
    pub async fn get_authenticated_user_age_bracket(
        &self,
    ) -> Result<AuthenticatedUserAgeBracket, RequestError> {
        let url = format!("{BASE_URL}/users/authenticated/age-bracket");

        let components = RequestComponents {
            needs_auth: true,
            method: Method::GET,
            url,
            headers: None,
            body: None,
        };

        let data = self
            .make_request::<AuthenticatedUserAgeBracket>(components, false)
            .await?;
        Ok(data)
    }

    /// **MUST AUTHENTICATE**
    ///
    /// Gets the authenticated user's country code
    ///
    /// # Errors
    ///
    /// This function will error if:
    /// - You do not have a `.ROBLOSECURITY` cookie set
    /// - The endpoint responds with an error
    ///
    /// Possible error responses include:
    /// - Status 401 code 0: Authorization denied
    pub async fn get_authenticated_user_country_code(
        &self,
    ) -> Result<AuthenticatedUserCountryCode, RequestError> {
        let url = format!("{BASE_URL}/users/authenticated/country-code");

        let components = RequestComponents {
            needs_auth: true,
            method: Method::GET,
            url,
            headers: None,
            body: None,
        };

        let data = self
            .make_request::<AuthenticatedUserCountryCode>(components, false)
            .await?;
        Ok(data)
    }

    /// **MUST AUTHENTICATE**
    ///
    /// Gets the authenticated user's group roles
    ///
    /// # Errors
    ///
    /// This function will error if:
    /// - You do not have a `.ROBLOSECURITY` cookie set
    /// - The endpoint responds with an error
    ///
    /// Possible error responses include:
    /// - Status 401 code 0: Authorization denied
    pub async fn get_authenticated_user_roles(
        &self,
    ) -> Result<AuthenticatedUserRoles, RequestError> {
        let url = format!("{BASE_URL}/users/authenticated/roles");

        let components = RequestComponents {
            needs_auth: true,
            method: Method::GET,
            url,
            headers: None,
            body: None,
        };

        let data = self
            .make_request::<AuthenticatedUserRoles>(components, false)
            .await?;
        Ok(data)
    }

    /// **MUST AUTHENTICATE**
    ///
    /// Validates a `display_name`. The authenticated user's `user_id`
    /// must be provided.
    ///
    /// # Errors
    ///
    /// This function will error if:
    /// - You do not have a `.ROBLOSECURITY` cookie set
    /// - The endpoint responds with an error
    ///
    /// Possible error responses include:
    /// - Status 400 code 1: Display name is too short
    /// - Status 400 code 2: Display name is too long
    /// - Status 400 code 3: Display name has invalid characters
    /// - Status 400 code 4: Display name has been moderated
    /// - Status 401 code 0: Authorization denied
    /// - Status 403 code 7: The user ID is invalid
    /// - Status 429 code 5: Display name updates have been throttled
    pub async fn validate_user_display_name(
        &self,
        user_id: usize,
        display_name: String,
    ) -> Result<(), RequestError> {
        let url =
            format!("{BASE_URL}/users/{user_id}/display-names/validate?displayName={display_name}");

        let components = RequestComponents {
            needs_auth: true,
            method: Method::GET,
            url,
            headers: None,
            body: None,
        };

        self.make_request::<serde_json::Value>(components, false)
            .await?;

        Ok(())
    }
}
