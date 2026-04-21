use std::collections::HashMap;
use std::fs::File;
use vaultrs::client::VaultClient;

mod utils;
use utils::*;

#[tokio::main]
pub async fn move_secrets(
    vault_client: &VaultClient,
    mount: &str,
    source_path: &str,
    dest_path: &str,
    destroy: &bool,
) {
    let mut moved_secrets_list: Vec<String> = vec![];

    assert_ne!(
        source_path, dest_path,
        "Source & destination paths must be different"
    );

    if source_path.ends_with("/") {
        if dest_path.ends_with("/") {
            moved_secrets_list = move_folder(vault_client, mount, source_path, dest_path).await;
        } else {
            panic!("If you want to move a folder, destination path must ends with '/'");
        }
    } else {
        moved_secrets_list = move_secret(vault_client, mount, source_path, dest_path).await;
    }

    // Destroy moved secrets
    if *destroy && !moved_secrets_list.is_empty() {
        for secret in moved_secrets_list {
            destroy_secret(vault_client, mount, &secret)
                .await
                .unwrap_or_else(|e| panic!("Cannot delete secret {source_path} : {e}"));
        }
    }
}

#[tokio::main]
pub async fn backup_secrets(
    vault_client: &VaultClient,
    mount: &str,
    file: &str,
    source_path: &str,
) {
    let secrets_data: HashMap<String, HashMap<String, String>> =
        list_secrets(vault_client, mount, source_path)
            .await
            .unwrap_or_else(|e| panic!("Cannot list folder {source_path} : {e}"));

    let f = File::create(file).unwrap_or_else(|e| panic!("Cannot create file {file} : {e}"));
    serde_json::to_writer_pretty(f, &secrets_data)
        .unwrap_or_else(|e| panic!("Cannot write into file {file} : {e}"));
}

#[tokio::main]
pub async fn destroy_secrets(vault_client: &VaultClient, mount: &str, source_path: &str) {
    if source_path.ends_with("/") {
        let secrets_list: HashMap<String, HashMap<String, String>> =
            list_secrets(vault_client, mount, source_path)
                .await
                .unwrap_or_else(|e| panic!("Cannot list folder {source_path} : {e}"));
        for secret in secrets_list.keys() {
            destroy_secret(vault_client, mount, secret)
                .await
                .unwrap_or_else(|e| panic!("Cannot delete secret {source_path} : {e}"));
        }
    } else {
        destroy_secret(vault_client, mount, source_path)
            .await
            .unwrap_or_else(|e| panic!("Cannot delete secret {source_path} : {e}"));
    }
}

#[tokio::main]
pub async fn restore_secrets(vault_client: &VaultClient, mount: &str, file: &str) {}
