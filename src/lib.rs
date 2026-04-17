use vaultrs::client::{VaultClient, VaultClientSettingsBuilder, Client};
use vaultrs::error::ClientError;
use vaultrs::kv2;
use std::collections::HashMap;


fn authenticator() -> VaultClient {
    let vault_url: String = std::env::var("VAULT_ADDR").unwrap();
    let vault_token: String = std::env::var("VAULT_TOKEN").unwrap();

    let settings = VaultClientSettingsBuilder::default()
        .address(vault_url)
        .token(vault_token)
        .build()
        .unwrap();

    VaultClient::new(settings).unwrap()

}

async fn move_folder(vault_client: &VaultClient, mount: &str, source_path: &str, dest_path: &str) {
    
    let folder_kv_list = kv2::list(vault_client, mount, source_path).await.unwrap();

    for kv in folder_kv_list {
        if kv.ends_with('/') {
            Box::pin(move_folder(vault_client, mount, &(source_path.to_owned() + &kv), &(dest_path.to_owned() + &kv))).await;
        }
        else {
            println!("move {}", kv);
            move_secret(vault_client, mount, &(source_path.to_owned() + &kv), &(dest_path.to_owned() + &kv)).await;
        }
    }
}

async fn move_secret(vault_client: &VaultClient, mount: &str, source_path: &str, dest_path: &str) {

    let secret: HashMap<String, String> = kv2::read(vault_client, mount, source_path).await.unwrap();
    
    kv2::set(
        vault_client,
        mount,
        dest_path ,
        &secret,
    ).await.unwrap_or_else(|e| panic!("Cannot create secret : {e}"));
}

// async fn destroy_secret(vault_client: &VaultClient, mount: &str, path: &str) {


// }

#[tokio::main]
pub async fn move_secrets(
    mount: &str,
    source_path: &str,
    dest_path: &str,
    destroy: &bool
) {

    let vault_client: VaultClient = authenticator();
    
    assert_ne!(source_path, dest_path, "Source & destination paths must be different");

    if source_path.ends_with("/") {
        if dest_path.ends_with("/") {
            move_folder(&vault_client, &mount, &source_path, &dest_path).await;
        }
        else {
            panic!("If you want to move a folder, destination path must ends with '/'");
        }
    } else {
        move_secret(&vault_client, &mount, &source_path, &dest_path).await;
    }

    if *destroy {
        println!("TO DO");
    }
}