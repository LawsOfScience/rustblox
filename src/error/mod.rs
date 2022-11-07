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
    /// There was an error sending the request
    RequestError(String, String),
    /// The endpoint returned a 400-class error code
    ClientError(String, String),
    /// The endpoint returned a 500-class error code
    ServerError(String, String),
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
            Self::RequestError(url, err) => {
                f.write_str(format!("Had an error sending the request to {url}:\n{err}").as_str())
            },
            Self::ClientError(url, err) => {
                f.write_str(format!("{url} returned a client error:\n{err}").as_str())
            },
            Self::ServerError(url, err) => {
                f.write_str(format!("{url} had an internal server error:\n{err}").as_str())
            }
        }
    }
}

impl StdErr for ClientError {}
impl StdErr for RequestError {}
