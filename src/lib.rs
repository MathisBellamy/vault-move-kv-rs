use std::collections::HashMap;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::error::ClientError;
use vaultrs::kv2;

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

async fn move_folder(
    vault_client: &VaultClient,
    mount: &str,
    source_path: &str,
    dest_path: &str,
) -> Vec<String> {
    let folder_kv_list: Vec<String> = kv2::list(vault_client, mount, source_path).await.unwrap();
    let mut moved_secrets: Vec<String> = Vec::new();

    for kv in folder_kv_list {
        if kv.ends_with('/') {
            let mut sub_moved = Box::pin(move_folder(
                vault_client,
                mount,
                &(source_path.to_owned() + &kv),
                &(dest_path.to_owned() + &kv),
            ))
            .await;
            moved_secrets.append(&mut sub_moved);
        } else {
            move_secret(
                vault_client,
                mount,
                &(source_path.to_owned() + &kv),
                &(dest_path.to_owned() + &kv),
            )
            .await;
            moved_secrets.push(source_path.to_owned() + &kv);
        }
    }
    moved_secrets
}

async fn move_secret(
    vault_client: &VaultClient,
    mount: &str,
    source_path: &str,
    dest_path: &str,
) -> Vec<String> {
    let secret: HashMap<String, String> =
        kv2::read(vault_client, mount, source_path).await.unwrap();

    kv2::set(vault_client, mount, dest_path, &secret)
        .await
        .unwrap_or_else(|e| panic!("Cannot create secret : {e}"));

    vec![String::from(source_path)]
}

async fn destroy_secret(vault_client: &VaultClient, mount: &str, path: &str) {
    kv2::delete_metadata(vault_client, mount, path)
        .await
        .unwrap_or_else(|e| panic!("Cannot destroy secret {path} : {e}"));
}

#[tokio::main]
pub async fn move_secrets(mount: &str, source_path: &str, dest_path: &str, destroy: &bool) {
    let vault_client: VaultClient = authenticator()
        .unwrap_or_else(|e: ClientError| panic!("Cannot authenticate to Vault : {e}"));

    let mut moved_secrets_list: Vec<String> = vec![];

    assert_ne!(
        source_path, dest_path,
        "Source & destination paths must be different"
    );

    if source_path.ends_with("/") {
        if dest_path.ends_with("/") {
            moved_secrets_list = move_folder(&vault_client, mount, source_path, dest_path).await;
        } else {
            panic!("If you want to move a folder, destination path must ends with '/'");
        }
    } else {
        moved_secrets_list = move_secret(&vault_client, mount, source_path, dest_path).await;
    }
    if *destroy && !moved_secrets_list.is_empty() {
        for secret in moved_secrets_list {
            destroy_secret(&vault_client, mount, &secret).await;
        }
    }
}
