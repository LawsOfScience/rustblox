#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;

pub mod client;
pub mod error;
pub mod structs;

mod routes;

#[cfg(test)]
mod tests;
