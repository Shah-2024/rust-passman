use clap::{Parser, Subcommand};

/// Simple CLI Password Manager
#[derive(Parser)]
#[command(name = "PassMan")]
#[command(about = "A secure CLI password manager built in Rust", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize the encrypted vault
    Init,

    /// Add a new credential
    Add {
        #[arg(short, long)]
        site: String,

        #[arg(short, long)]
        username: String,

        #[arg(short, long)]
        password: String,
    },

    /// View all credentials
    View,

    /// Delete a credential by site
    Delete {
        #[arg(short, long)]
        site: String,
    },
}