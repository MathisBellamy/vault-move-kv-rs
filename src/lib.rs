use std::collections::HashMap;
use std::fs::File;
use vaultrs::client::VaultClient;
use vaultrs::kv2;

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

    if source_path.ends_with('/') && !dest_path.ends_with('/') {
        panic!("If you want to move a folder, destination path must ends with '/'");
    }

    let all_secrets: HashMap<String, HashMap<String, String>> =
        list_secrets(vault_client, mount, source_path)
            .await
            .unwrap_or_else(|e| panic!("Cannot list folder {source_path} : {e}"));

    for (secret_path, secret_data) in all_secrets.into_iter() {
        let kv_path_without_source: String = secret_path.replacen(source_path, "", 1);
        let dest_path: String = format!("{}{}", dest_path, kv_path_without_source);

        kv2::set(vault_client, mount, dest_path.as_str(), &secret_data)
            .await
            .unwrap_or_else(|e| panic!("Cannot create secret {dest_path} : {e}"));
        moved_secrets_list.push(secret_path);
    }

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
pub async fn restore_secrets(vault_client: &VaultClient, mount: &str, file: &str) {
    let f = File::open(file).unwrap_or_else(|e| panic!("Cannot open file {file} : {e}"));

    let json_formatted_content: serde_json::Value =
        serde_json::from_reader(f).expect("JSON was not formatted correctly");

    match json_formatted_content.as_object() {
        Some(secrets) => {
            for (secret_path, secret_data_obj) in secrets {
                match secret_data_obj.as_object() {
                    Some(secret_data) => {
                        kv2::set(vault_client, mount, secret_path, secret_data)
                            .await
                            .unwrap_or_else(|e| panic!("Cannot create secret {secret_path} : {e}"));
                    }
                    None => println!("{secret_path} is not a JSON object"),
                }
            }
        }
        None => println!("File is not well JSON formatted"),
    }
}
