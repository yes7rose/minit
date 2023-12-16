use dependencies_sync::bson::Document;
use dependencies_sync::log;

use auth::jwt::hash_password;
use manage_define::general_field_ids::ID_FIELD_ID;

use account_module::ids_codes::manage_ids::ACCOUNTS_MANAGE_ID;
use account_module::ids_codes::field_ids::ACCOUNTS_PASSWORD_FIELD_ID;

/// 初始化根口令
pub async fn init_root_password(root_id: &String, passswd: &Option<String>) {
    let hashed_passwd = if passswd.is_some() {
        hash_password(passswd.as_ref().unwrap()).await.unwrap()
    } else {
        hash_password(&"root".to_string()).await.unwrap()
    };

    let mut query_doc = Document::new();
    query_doc.insert(ID_FIELD_ID.to_string(), root_id.clone());

    let mut modify_doc = Document::new();
    modify_doc.insert(ACCOUNTS_PASSWORD_FIELD_ID.to_string(), hashed_passwd);

    match entity::update_entity_field(
        &ACCOUNTS_MANAGE_ID.to_string(),
        query_doc,
        &mut modify_doc,
        root_id,
    )
    .await
    {
        Err(r) => log::error!("{}", r.details()),
        _ => {}
    };
}
