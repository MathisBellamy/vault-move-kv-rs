use clap::{Arg, Args, Command, Parser, Subcommand};

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

        #[arg(long, short = 'p', help = "Password to encrypt data", required = true)]
        password: String,

        source_path: String,
    },
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Move {
            mount,
            destroy,
            source_path,
            dest_path,
        }) => {
            move_secrets(mount, source_path, dest_path, destroy);
        }
        Some(Commands::Backup {
            mount,
            password,
            source_path,
        }) => {
            println!("Backup");
            backup_secrets(mount, password, source_path);
        }
        None => {
            println!("None");
        }
    }

    // let cmd: clap::ArgMatches = Command::new(env!("CARGO_PKG_NAME"))
    //     .version(env!("CARGO_PKG_VERSION"))
    //     .about("Vault CLI")
    //     .subcommand()
    //     .arg(
    //         Arg::new("mount")
    //             .short('m')
    //             .long("mount")
    //             .value_name("VAULT_MOUNT")
    //             .help("Mount")
    //             .num_args(1)
    //             .required(true),
    //     )
    //     .arg(
    //         Arg::new("destroy")
    //             .short('d')
    //             .long("destroy")
    //             .value_name("VAULT_DESTROY")
    //             .action(clap::ArgAction::SetTrue)
    //             .help("If you want to destroy moved secrets")
    //             .required(false),
    //     )
    //     .arg(
    //         Arg::new("source_path")
    //             .value_name("VAULT_SOURCE_PATH")
    //             .help("Source path")
    //             .required(true),
    //     )
    //     .arg(
    //         Arg::new("dest_path")
    //             .value_name("VAULT_DEST_PATH")
    //             .help("Destination path")
    //             .required(true),
    //     )
    //     .get_matches();

    // move_secrets(
    //     cmd.get_one::<String>("mount").unwrap(),
    //     cmd.get_one::<String>("source_path").unwrap(),
    //     cmd.get_one::<String>("dest_path").unwrap(),
    //     cmd.get_one::<bool>("destroy").unwrap(),
    // )
}
