use dependencies_sync::bson::Document;
use dependencies_sync::log;

use auth::jwt::hash_password;
use dependencies_sync::rust_i18n::{self, t};
use manage_define::general_field_ids::ID_FIELD_ID;

use account_module::ids_codes::manage_ids::ACCOUNTS_MANAGE_ID;
use account_module::ids_codes::field_ids::ACCOUNTS_PASSWORD_FIELD_ID;

/// zh: 初始化根口令
/// en: Initialize the root password
pub async fn init_root_password(root_id: &str, passswd: &Option<String>) {
    let hashed_passwd = if passswd.is_some() {
        hash_password(passswd.as_ref().unwrap()).await.unwrap()
    } else {
        hash_password(&"root".to_string()).await.unwrap()
    };

    let mut query_doc = Document::new();
    query_doc.insert(ID_FIELD_ID.to_string(), root_id);

    let mut modify_doc = Document::new();
    modify_doc.insert(ACCOUNTS_PASSWORD_FIELD_ID.to_string(), hashed_passwd);

    if let Err(r) =  entity::update_entity_field(
        ACCOUNTS_MANAGE_ID,
        query_doc,
        &mut modify_doc,
        root_id,
    )
    .await
    {
        log::error!("{}: {}", t!("初始化根口令失败"), r.details())
    };
}
