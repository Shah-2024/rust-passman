use crate::models::{Vault, Credential};
use dirs::home_dir;
use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::PathBuf;

const VAULT_FILENAME: &str = ".rust_passman.vault.json";

/// Get path to the vault file
fn vault_path() -> PathBuf {
    let mut path = home_dir().expect("Failed to get home directory");
    path.push(VAULT_FILENAME);
    path
}

/// Initialize an empty vault file
pub fn init_vault() -> io::Result<()> {
    let path = vault_path();
    if path.exists(){
        println!("Vault already exists at: {:?}", path);
        return Ok(());
    }

    let empty = Vault::default();
    let data = serde_json::to_string_pretty(&empty)?;
    fs::write(&path, data)?;
    println!("Vault initialized at: {:?}", path);
    Ok(())
}

/// Load vault from file
pub fn load_vault() -> io::Result<Vault> {
    let path = vault_path();
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Vault not found"));
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let vault: Vault = serde_json::from_str(&contents)?;
    Ok(vault)
}

/// Save vault to file
pub fn save_vault(vault: &Vault) -> io::Result<()> {
    let path = vault_path();
    let data = serde_json::to_string_pretty(vault)?;
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

/// Add a new credential
pub fn add_credential(site: String, username: String, password: String) -> io::Result<()> {
    let mut vault = load_vault().unwrap_or_default();
    vault.credentials.push(Credential {site, username, password});
    save_vault(&vault)?;
    println!("Credential added.");
    Ok(())
}

/// View all credentials
pub fn view_credentials() -> io::Result<()> {
    let vault = load_vault()?;
    for cred in vault.credentials {
        println!("Site: {}\nUsername: {}\nPassword: {}\n", cred.site, cred.username, cred.password);
    }
    Ok(())
}

/// Delete a credential by site name
pub fn delete_credential(site: &str) -> io::Result<()> {
    let mut vault = load_vault()?;
    let before = vault.credentials.len();
    vault.credentials.retain(|c| c.site != site);

    if vault.credentials.len() == before {
        println!("No credential found for size: {}", site);
    } else {
        save_vault(&vault)?;
        println!("Credential deleted.");
    }
    Ok(())
}