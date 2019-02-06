//! Fitbit Web API client

#![deny(warnings)]

mod args;
mod client;

/// Possible commands for the client to execute.
#[derive(PartialEq)]
pub(crate) enum Command {
    GetActivitySummary,
    GetAuthToken(String, String),
    GetBadges,
    GetDevices,
    GetProfile,
}

fn main() {
    let command = args::parse();

    match command {
        Command::GetActivitySummary => client::get_activity_summary(),
        Command::GetAuthToken(id, secret) => client::get_auth_token(id, secret),
        Command::GetBadges => client::get_badges(),
        Command::GetDevices => client::get_devices(),
        Command::GetProfile => client::get_profile(),
    }
}