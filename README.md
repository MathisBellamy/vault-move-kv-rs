# What is it?

This tool allows to do the following actions on a Vault/OpenBao instance :
* Move a kv or a folder
* Destroy a kv or a folder
* Backup a kv or a folder in a json file
* Restore a json file in a mount point

# Prerequisites

This tool requires the following env variables:
```bash
export VAULT_ADDR=<changeme>
export VAULT_TOKEN=<changeme>
```

# How does this function?

Get the executable or compile it, the signature of the executable is as follows:
```
Vault/OpenBao CLI to move, destroy, backup and restore

Usage: 
Move    > vault-move-kv-rs move [--destroy] --mount <MOUNT> <SOURCE_PATH> <DEST_PATH>
Destroy > vault-move-kv-rs destroy --mount <MOUNT> <SOURCE_PATH>
Backup  > vault-move-kv-rs backup --mount <MOUNT> --file <FILE> <SOURCE_PATH>
Restore > vault-move-kv-rs restore --mount <MOUNT> --file <FILE>
```
