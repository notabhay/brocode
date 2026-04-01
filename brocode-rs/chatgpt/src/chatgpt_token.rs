use brocode_core::AuthManager;
use brocode_login::token_data::TokenData;
use std::path::Path;
use std::sync::LazyLock;
use std::sync::RwLock;

use brocode_core::auth::AuthCredentialsStoreMode;

static CHATGPT_TOKEN: LazyLock<RwLock<Option<TokenData>>> = LazyLock::new(|| RwLock::new(None));

pub fn get_chatgpt_token_data() -> Option<TokenData> {
    CHATGPT_TOKEN.read().ok()?.clone()
}

pub fn set_chatgpt_token_data(value: TokenData) {
    if let Ok(mut guard) = CHATGPT_TOKEN.write() {
        *guard = Some(value);
    }
}

/// Initialize the ChatGPT token from auth.json file
pub async fn init_chatgpt_token_from_auth(
    brocode_home: &Path,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
) -> std::io::Result<()> {
    let auth_manager = AuthManager::new(
        brocode_home.to_path_buf(),
        /*enable_brocode_api_key_env*/ false,
        auth_credentials_store_mode,
    );
    if let Some(auth) = auth_manager.auth().await {
        let token_data = auth.get_token_data()?;
        set_chatgpt_token_data(token_data);
    }
    Ok(())
}
