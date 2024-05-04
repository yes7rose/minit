use dependencies_sync::bson::doc;
use dependencies_sync::log;
use dependencies_sync::mongodb::Database;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::toml::map::Map;
use dependencies_sync::toml::Value;

use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;

use define_utils as utils;

pub async fn init_manages_db(
    db: &Database,
    tomls: &Vec<Map<String, Value>>,
    root_id: &String,
    root_group_id: &String,
) {
    for map in tomls {
        let manage_id = match utils::get_id(map) {
            Some(m) => m.to_string(),
            None => continue,
        };

        let manage_name = match utils::get_name_map(map) {
            Some(m) => m,
            None => continue,
        };
        let manage_schema = match utils::get_schema(map) {
            Some(s) => s,
            None => continue,
        };
        let hard_coded = match utils::get_hard_coded(map) {
            Some(h) => h,
            None => false,
        };

        log::info!("\t{}: {} {}", t!("开始创建管理"), manage_id, manage_name);

        let mut manage_doc = doc! {
            "_id": manage_id.clone(),
            ID_FIELD_ID.to_string(): manage_id.clone(),
            NAME_MAP_FIELD_ID.to_string(): manage_name.clone(),
            MANAGES_SCHEMA_FIELD_ID.to_string(): manage_schema,
            MANAGES_HARD_CODED_FIELD_ID.to_string(): hard_coded,
        };

        // 添加管理实体
        if entity::entity_exists(
            MANAGES_MANAGE_ID.to_string().as_str(),
            &doc! {ID_FIELD_ID.to_string(): manage_id.clone()},
        )
        .await
        .is_none()
        {
            match entity::insert_entity(
                &MANAGES_MANAGE_ID.to_string(),
                &mut manage_doc,
                root_id,
                root_group_id,
            )
            .await
            {
                Ok(r) => {
                    log::info!(
                        "\t{}: {} {}",
                        t!("添加管理实体成功"),
                        manage_id,
                        manage_name
                    );
                    Some(r)
                }
                Err(e) => {
                    log::error!(
                        "\t{}:  {} {} {}",
                        t!("添加管理实体失败"),
                        manage_id,
                        manage_name,
                        e.details()
                    );
                    None
                }
            };
        } else {
            log::warn!("\t{}: {} {} ", t!("管理实体已经存在"), manage_id, manage_name);
            continue;
        }

        // 管理数据库集合已经存在，跳过
        if manage_id == MANAGES_MANAGE_ID.to_string() {
            continue;
        }

        // 创建集合
        match db.create_collection(&manage_id.clone(), None).await {
            Err(e) => {
                panic!("\t{}: {} {:?}", t!("创建管理集合失败"), manage_id, e)
            }
            _ => log::info!("\t{}: {}", t!("创建管理集合成功"), manage_id),
        }
        
        // TODO: 创建索引
    }
}
