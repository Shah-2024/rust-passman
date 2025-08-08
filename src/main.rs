mod cli;
mod models;
mod vault;

use cli::{Cli, Commands};
use clap::Parser;
use vault::*;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            if let Err(e) = init_vault() {
                eprintln!("Error: {}", e);
            }
        }
        Commands::Add { site, username, password } => {
            if let Err(e) = add_credential(site.clone(), username.clone(), password.clone()){
                eprintln!("Error: {}", e);
            }
        }
        Commands::View => {
            if let Err(e) = view_credentials() {
                eprintln!("Error: {}", e);
            }
        }
        Commands::Delete { site } => {
            if let Err(e) = delete_credential(site) {
                eprintln!("Error: {}", e);
            }
        }
    }
}
