//! Fitbit Web API client
//!
#![deny(warnings)]

mod args;
mod client;

/// Possible commands for the client to execute.
#[derive(PartialEq)]
pub(crate) enum Command {
    GetAuthToken(String, String),
    GetDevices,
}

fn main() {
    let command = args::parse();

    match command {
        Command::GetAuthToken(id, secret) => client::get_auth_token(id, secret),
        Command::GetDevices => client::get_devices(),
    }
}