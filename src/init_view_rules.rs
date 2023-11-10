use dependencies_sync::bson::{self, doc};
use dependencies_sync::toml::map::Map;
use dependencies_sync::toml::Value;
use dependencies_sync::log;

use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;

use define_utils as utils;

pub async fn init_view_rules(
    tomls: &Vec<Map<String, Value>>,
    root_id: &String,
    root_group_id: &String,
) {
    for map in tomls {
        let rule_id = utils::get_id(map).unwrap();
        let rule_name = match utils::get_name_map(map) {
            Some(m) => m,
            None => {
                log::error!("取得映像规则名失败: {} ", rule_id);
                continue;
            }
        };

        let view_rules = match utils::get_init_view_rules(map) {
            Some(m) => m,
            None => continue,
        };

        let mut rulse_doc = doc! {
            "_id": rule_id.to_string(),
            ID_FIELD_ID.to_string(): rule_id.to_string(),
            NAME_MAP_FIELD_ID.to_string(): rule_name.clone(),
            VIEW_RULES_MANAGE_FIELD_ID.to_string(): bson::to_bson(&view_rules.manage).unwrap(),
            VIEW_RULES_COLLECTION_FIELD_ID.to_string(): bson::to_bson(&view_rules.collection).unwrap(),
            VIEW_RULES_ENTITY_FIELD_ID.to_string(): bson::to_bson(&view_rules.schema).unwrap(),
        };

        match entity::insert_entity(
            &VIEW_RULES_MANAGE_ID.to_string(),
            &mut rulse_doc,
            root_id,
            root_group_id,
        )
        .await
        {
            Ok(_r) => continue,
            Err(e) => println!("{} {}", e.operation(), e.details()),
        }
    }
}
