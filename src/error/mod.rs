use std::error::Error as StdErr;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[non_exhaustive]
pub enum ClientError {
    /// The `.ROBLOSECURITY` cookie is invalid.
    InvalidCookie,
    /// The login process failed.
    LoginFailed(String),
    /// Setting the `.ROBLOSECURITY` cookie failed.
    CookieError(String),
    /// The reqwest client builder failed.
    ReqwestBuildError(String),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum RequestError {
    /// The user tried to use an endpoint that requires authentication
    /// without being authenticated
    NotAuthenticated,
    /// Automatic client reauthentication failed
    ReauthenticationFailed(String),
    /// The `.ROBLOSECURITY` cookie in use expired
    ExpiredCookie,
    /// There was an error sending the request
    RequestError(String, String),
    /// The server returned a 400-class error code (client error).
    /// Contains the url, status code, and a collection of errors
    ClientError(String, u16, RobloxApiErrors),
    /// The server returned a 500-class error code (server error)
    /// Contains the status code
    ServerError(u16),
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCookie => f.write_str("The provided .ROBLOSECURITY token is invalid"),
            Self::CookieError(err) => f.write_str(
                format!("Had an error setting the .ROBLOSECURITY cookie in reqwest:\n{err}")
                    .as_str(),
            ),
            Self::LoginFailed(err) => {
                f.write_str(format!("Logging into the Roblox API failed:\n{err}").as_str())
            }
            Self::ReqwestBuildError(err) => {
                f.write_str(format!("Had an error building the Reqwest client:\n{err}").as_str())
            }
        }
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotAuthenticated => f.write_str("You need to be logged in to use this endpoint!"),
            Self::ReauthenticationFailed(msg) => {
                f.write_str(format!("Automatic reauthentication failed:\n{msg}").as_str())
            }
            Self::ExpiredCookie => f.write_str("The .ROBLOSECURITY cookie expired."),
            Self::RequestError(url, err) => {
                f.write_str(format!("Had an error sending the request to {url}:\n{err}").as_str())
            }
            Self::ClientError(url, status_code, errors) => f.write_str(
                format!(
                    "{url} returned status code {status_code}\n{}",
                    errors.to_string()
                )
                .as_str(),
            ),
            Self::ServerError(status_code) => {
                f.write_str(format!("Server returned status code {status_code}").as_str())
            }
        }
    }
}

impl StdErr for ClientError {}
impl StdErr for RequestError {}

#[derive(Deserialize, Debug)]
pub struct RobloxApiError {
    pub code: i16,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct RobloxApiErrors {
    pub errors: Vec<RobloxApiError>,
}

impl ToString for RobloxApiError {
    fn to_string(&self) -> String {
        format!("Error code {}: \"{}\"", self.code, self.message)
    }
}

impl ToString for RobloxApiErrors {
    fn to_string(&self) -> String {
        let mut format_str = String::from("Roblox API Errors:\n");
        for error in &self.errors {
            format_str += " - ";
            format_str += error.to_string().as_str();
            format_str += "\n";
        }
        format_str
    }
}
