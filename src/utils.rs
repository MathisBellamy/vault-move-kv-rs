use std::collections::HashMap;
use std::error::Error;
use vaultrs::client::VaultClient;
use vaultrs::kv2;

pub async fn list_secrets(
    vault_client: &VaultClient,
    mount: &str,
    path: &str,
) -> Result<HashMap<String, HashMap<String, String>>, Box<dyn Error>> {
    let folder_kv_list: Vec<String> = if path.ends_with('/') {
        kv2::list(vault_client, mount, path).await?
    } else {
        vec![path.to_string()]
    };
    let mut all_data: HashMap<String, HashMap<String, String>> = HashMap::new();
    for kv in folder_kv_list {
        if kv.ends_with('/') {
            let sub_path = format!("{}{}", path, kv);
            let sub_data = Box::pin(list_secrets(vault_client, mount, &sub_path)).await?;
            all_data.extend(sub_data);
        } else if kv == path {
            let secret: HashMap<String, String> =
                kv2::read(vault_client, mount, path).await.unwrap();
            all_data.insert(path.to_string(), secret);
        } else {
            let full_path = format!("{}{}", path, kv);
            let secret: HashMap<String, String> =
                kv2::read(vault_client, mount, &full_path).await.unwrap();
            all_data.insert(full_path, secret);
        }
    }
    Ok(all_data)
}

pub async fn destroy_secret(
    vault_client: &VaultClient,
    mount: &str,
    path: &str,
) -> Result<(), Box<dyn Error>> {
    kv2::delete_metadata(vault_client, mount, path).await?;
    Ok(())
}
