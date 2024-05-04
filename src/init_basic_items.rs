use dependencies_sync::bson::doc;
use dependencies_sync::toml::map::Map;
use dependencies_sync::toml::Value;
use dependencies_sync::log;

use define_utils as utils;
use manage_define::general_field_ids::ID_FIELD_ID;

pub async fn init_basic_items(
    tomls: &Vec<Map<String, Value>>,
    root_id: &str,
    root_group_id: &str,
) {
    for map in tomls {
        let manage_id = utils::get_id(map).unwrap();
        log::info!("\t开始插入数据到集合: {}", manage_id);

        if let Some(items) = utils::get_init_items(map) {
            let length = items.len();
            for mut item in items {
                // 检查是否重复
                let id = item.get_str(ID_FIELD_ID.to_string()).unwrap();
                let query_doc = doc!(ID_FIELD_ID.to_string(): id);
                if entity::entity_exists(manage_id, &query_doc).await.is_some() {
                    continue;
                };

                if let Err(_r) =
                    entity::insert_entity(manage_id, &mut item, root_id, root_group_id)
                        .await
                {
                    log::warn!("插入记录失败, {}", _r.details());
                }
            }
            log::info!("\t\t插入数据个数: {}", length);
        }
    }
}
