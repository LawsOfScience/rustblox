#[macro_use]
extern crate serde;
extern crate log;

pub mod client;
pub mod error;
pub mod structs;

mod routes;

#[cfg(test)]
mod tests;
