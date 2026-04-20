use dict::Dict;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;

pub async fn list_folder(
    vault_client: &VaultClient,
    mount: &str,
    path: &str,
) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let folder_kv_list: Vec<String> = kv2::list(vault_client, mount, path).await?;

    let mut all_data: HashMap<String, Value> = HashMap::new();

    for kv in folder_kv_list {
        if kv.ends_with('/') {
            let sub_path = format!("{}{}", path, kv);
            let mut sub_data = Box::pin(list_folder(vault_client, mount, &sub_path)).await?;
            all_data.extend(sub_data);
        } else {
            let full_path = format!("{}{}", path, kv);
            let secret: HashMap<String, String> =
                kv2::read(vault_client, mount, &full_path).await.unwrap();
            all_data.insert(
                full_path,
                serde_json::to_value(secret)
                    .unwrap_or_else(|e| panic!("Cannot convert secret into JSON : {e}")),
            );
        }
    }
    Ok(all_data)
}
