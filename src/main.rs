use clap::{Arg, Command};
use lib::move_secrets;
mod lib;

fn main() {
    let cmd: clap::ArgMatches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Move secret(s) from one path to another in Vault")
        .arg(
            Arg::new("mount")
                .short('m')
                .long("mount")
                .value_name("VAULT_MOUNT")
                .help("Mount")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("destroy")
                .short('d')
                .long("destroy")
                .value_name("VAULT_DESTROY")
                .action(clap::ArgAction::SetTrue)
                .help("If you want to destroy moved secrets")
                .required(false),
        )
        .arg(
            Arg::new("source_path")
                .value_name("VAULT_SOURCE_PATH")
                .help("Source path")
                .required(true),
        )
        .arg(
            Arg::new("dest_path")
                .value_name("VAULT_DEST_PATH")
                .help("Destination path")
                .required(true),
        )
        .get_matches();

    move_secrets(
        cmd.get_one::<String>("mount").unwrap(),
        cmd.get_one::<String>("source_path").unwrap(),
        cmd.get_one::<String>("dest_path").unwrap(),
        cmd.get_one::<bool>("destroy").unwrap(),
    )
}
