use std::error::Error as StdErr;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[non_exhaustive]
pub enum ClientError {
    /// If the .ROBLOSECURITY token is invalid
    InvalidToken,
    /// If logging in fails
    LoginFailed(String),
    /// If setting the .ROBLOSECURITY cookie fails
    CookieError(String),
    /// If the reqwest client builder errors
    ReqwestBuildError(String),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum RequestError {
    /// If the user tries to use an endpoint that requires authentication
    /// without being authenticated
    NotAuthenticated,
    /// If there was an error sending the request
    RequestError(String, String)
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::InvalidToken =>
                f.write_str("The provided .ROBLOSECURITY token is invalid"),
            ClientError::CookieError(err) => {
                let message = format!(
                    "Had an error setting the .ROBLOSECURITY cookie in reqwest:\n{}",
                    err
                );
                f.write_str(message.as_str())
            },
            ClientError::LoginFailed(err) => {
                let message = format!(
                    "Logging into the Roblox API failed:\n{}",
                    err
                );
                f.write_str(message.as_str())
            },
            ClientError::ReqwestBuildError(err) => {
                let message = format!(
                    "Had an error building the Reqwest client:\n{}",
                    err
                );
                f.write_str(message.as_str())
            }
        }
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::NotAuthenticated =>
                f.write_str("You need to be logged in to use this endpoint!"),
            RequestError::RequestError(url, err) => {
                let message = format!(
                    "Had an error sending the request to {}:\n{}",
                    url, err
                );
                f.write_str(message.as_str())
            }
        }
    }
}

impl StdErr for ClientError {}
impl StdErr for RequestError {}
