use dependencies_sync::toml::map::Map;
use dependencies_sync::toml::Value;

use define_utils as utils;

pub async fn init_basic_items(
    tomls: &Vec<Map<String, Value>>,
    root_id: &String,
    root_group_id: &String,
) {
    for map in tomls {
        let manage_id = utils::get_id(map).unwrap();
        println!("\t开始插入数据到集合: {}", manage_id);

        if let Some(items) = utils::get_init_items(map) {
            let length = items.len();
            for mut item in items {
                if let Ok(_r) =
                    entity::insert_entity(&manage_id.to_string(), &mut item, root_id, root_group_id)
                        .await
                {
                    continue;
                } else {
                    println!("插入记录失败, {}", item);
                }
            }
            println!("\t\t插入数据个数: {}", length);
        }
    }
}
