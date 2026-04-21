use clap::{Parser, Subcommand};
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::error::ClientError;

use vault_move_kv_rs::*;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Vault CLI")]
#[command(subcommand_required = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Move {
        #[arg(long, short = 'm', help = "Mount", required = true)]
        mount: String,

        #[arg(long, short='d', help="To destroy moved secrets", required=false, action=clap::ArgAction::SetTrue)]
        destroy: bool,

        source_path: String,
        dest_path: String,
    },
    Backup {
        #[arg(long, short = 'm', help = "Mount", required = true)]
        mount: String,

        #[arg(long, short = 'f', help = "File to store data", required = true)]
        file: String,

        source_path: String,
    },
    Destroy {
        #[arg(long, short = 'm', help = "Mount", required = true)]
        mount: String,

        source_path: String,
    },
    Restore {
        #[arg(long, short = 'm', help = "Mount", required = true)]
        mount: String,

        #[arg(long, short = 'f', help = "File to store data", required = true)]
        file: String,
    },
}

fn authenticator() -> Result<VaultClient, ClientError> {
    let vault_url: String = std::env::var("VAULT_ADDR").unwrap();
    let vault_token: String = std::env::var("VAULT_TOKEN").unwrap();

    let settings = VaultClientSettingsBuilder::default()
        .address(vault_url)
        .token(vault_token)
        .build()
        .unwrap();

    let vault_client = VaultClient::new(settings)?;

    Ok(vault_client)
}

fn main() {
    let vault_client: VaultClient = authenticator()
        .unwrap_or_else(|e: ClientError| panic!("Cannot authenticate to Vault : {e}"));

    let args = Cli::parse();

    match &args.command {
        Some(Commands::Move {
            mount,
            destroy,
            source_path,
            dest_path,
        }) => {
            move_secrets(&vault_client, mount, source_path, dest_path, destroy);
        }
        Some(Commands::Backup {
            mount,
            file,
            source_path,
        }) => {
            backup_secrets(&vault_client, mount, file, source_path);
        }
        Some(Commands::Destroy { mount, source_path }) => {
            destroy_secrets(&vault_client, mount, source_path);
        }
        Some(Commands::Restore { mount, file }) => {
            restore_secrets(&vault_client, mount, file);
        }
        None => {
            println!("None");
        }
    }
}
