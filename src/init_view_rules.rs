use dependencies_sync::bson::{self, doc, Document};
use dependencies_sync::log;
use dependencies_sync::mongodb::IndexModel;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::toml::map::Map;
use dependencies_sync::toml::Value;

use entity::get_new_entity_id;
use manage_define::cashmere::ViewRuleLevel;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;

use define_utils as utils;

pub async fn init_view_rules(tomls: &Vec<Map<String, Value>>, root_id: &str, root_group_id: &str) {
    for map in tomls {
        let manage_id = utils::get_id(map).unwrap();
        let manage_name = match utils::get_name_map(map) {
            Some(m) => m,
            None => {
                log::error!("取得映像规则名失败: {} ", manage_id);
                continue;
            }
        };

        let view_rules = match utils::get_init_view_rules(map) {
            Some(m) => m,
            None => continue,
        };

        let mut docs: Vec<Document> = vec![];
        // 管理级
        for (group, rule) in view_rules.manage {
            let mut rule_doc = Document::new();
            rule_doc.insert(VIEW_RULES_GROUP_FIELD_ID.to_string(), group);
            rule_doc.insert(
                VIEW_RULES_LEVEL_FIELD_ID.to_string(),
                ViewRuleLevel::VlManage as i32,
            );
            rule_doc.insert(
                VIEW_RULES_SUBJECT_MANAGE_FIELD_ID.to_string(),
                manage_id.clone(),
            );
            // rule_doc.insert(VIEW_RULES_SUBJECT_FIELD_ID.to_string(), manage_id.clone());
            rule_doc.insert(
                VIEW_RULES_READ_RULE_FIELD_ID.to_string(),
                rule.read_rule as i32,
                // bson::to_bson(&rule.read_rule).unwrap(),
            );
            rule_doc.insert(
                VIEW_RULES_READ_FILTERS_FIELD_ID.to_string(),
                bson::to_bson(
                    &rule
                        .read_filters
                        .iter()
                        .map(|x| *x as i32)
                        .collect::<Vec<i32>>(),
                )
                .unwrap(),
            );
            rule_doc.insert(
                VIEW_RULES_WRITE_RULE_FIELD_ID.to_string(),
                // bson::to_bson(&rule.write_rule).unwrap(),
                rule.write_rule as i32,
            );
            rule_doc.insert(
                VIEW_RULES_WRITE_FILTERS_FIELD_ID.to_string(),
                bson::to_bson(
                    &rule
                        .write_filters
                        .iter()
                        .map(|x| *x as i32)
                        .collect::<Vec<i32>>(),
                )
                .unwrap(),
            );
            docs.push(rule_doc);
        }

        // 集合级
        for (group, rule) in view_rules.collection {
            let mut rule_doc = Document::new();
            rule_doc.insert(VIEW_RULES_GROUP_FIELD_ID.to_string(), group);
            rule_doc.insert(
                VIEW_RULES_LEVEL_FIELD_ID.to_string(),
                ViewRuleLevel::VlCollection as i32,
            );
            rule_doc.insert(
                VIEW_RULES_SUBJECT_MANAGE_FIELD_ID.to_string(),
                manage_id.clone(),
            );
            rule_doc.insert(
                VIEW_RULES_READ_RULE_FIELD_ID.to_string(),
                rule.read_rule as i32,
                // bson::to_bson(&rule.read_rule).unwrap(),
            );
            rule_doc.insert(
                VIEW_RULES_READ_FILTERS_FIELD_ID.to_string(),
                bson::to_bson(
                    &rule
                        .read_filters
                        .iter()
                        .map(|x| *x as i32)
                        .collect::<Vec<i32>>(),
                )
                .unwrap(),
            );
            rule_doc.insert(
                VIEW_RULES_WRITE_RULE_FIELD_ID.to_string(),
                // bson::to_bson(&rule.write_rule).unwrap(),
                rule.write_rule as i32,
            );
            rule_doc.insert(
                VIEW_RULES_WRITE_FILTERS_FIELD_ID.to_string(),
                bson::to_bson(
                    &rule
                        .write_filters
                        .iter()
                        .map(|x| *x as i32)
                        .collect::<Vec<i32>>(),
                )
                .unwrap(),
            );
            docs.push(rule_doc);
        }

        // 实体集
        for (group, rule) in view_rules.entity {
            let mut rule_doc = Document::new();
            rule_doc.insert(VIEW_RULES_GROUP_FIELD_ID.to_string(), group);
            rule_doc.insert(
                VIEW_RULES_LEVEL_FIELD_ID.to_string(),
                ViewRuleLevel::VlEntity as i32,
            );
            rule_doc.insert(
                VIEW_RULES_SUBJECT_MANAGE_FIELD_ID.to_string(),
                manage_id.clone(),
            );
            // rule_doc.insert(VIEW_RULES_SUBJECT_FIELD_ID.to_string(), manage_id.clone());
            rule_doc.insert(
                VIEW_RULES_READ_RULE_FIELD_ID.to_string(),
                rule.read_rule as i32,
                // bson::to_bson(&rule.read_rule).unwrap(),
            );
            rule_doc.insert(
                VIEW_RULES_READ_FILTERS_FIELD_ID.to_string(),
                bson::to_bson(
                    &rule
                        .read_filters
                        .iter()
                        .map(|x| *x as i32)
                        .collect::<Vec<i32>>(),
                )
                .unwrap(),
            );
            rule_doc.insert(
                VIEW_RULES_WRITE_RULE_FIELD_ID.to_string(),
                // bson::to_bson(&rule.write_rule).unwrap(),
                rule.write_rule as i32,
            );
            rule_doc.insert(
                VIEW_RULES_WRITE_FILTERS_FIELD_ID.to_string(),
                bson::to_bson(
                    &rule
                        .write_filters
                        .iter()
                        .map(|x| *x as i32)
                        .collect::<Vec<i32>>(),
                )
                .unwrap(),
            );
            docs.push(rule_doc);
        }

        // 字段级
        for (field, rules) in view_rules.fields {
            for (group, rule) in rules {
                let mut rule_doc = Document::new();
                rule_doc.insert(VIEW_RULES_GROUP_FIELD_ID.to_string(), group);
                rule_doc.insert(
                    VIEW_RULES_LEVEL_FIELD_ID.to_string(),
                    ViewRuleLevel::VlField as i32,
                );
                rule_doc.insert(
                    VIEW_RULES_SUBJECT_MANAGE_FIELD_ID.to_string(),
                    manage_id.clone(),
                );
                rule_doc.insert(VIEW_RULES_SUBJECT_FIELD_ID.to_string(), field.clone());
                rule_doc.insert(
                    VIEW_RULES_READ_RULE_FIELD_ID.to_string(),
                    rule.read_rule as i32,
                    // bson::to_bson(&rule.read_rule).unwrap(),
                );
                rule_doc.insert(
                    VIEW_RULES_READ_FILTERS_FIELD_ID.to_string(),
                    bson::to_bson(
                        &rule
                            .read_filters
                            .iter()
                            .map(|x| *x as i32)
                            .collect::<Vec<i32>>(),
                    )
                    .unwrap(),
                );
                rule_doc.insert(
                    VIEW_RULES_WRITE_RULE_FIELD_ID.to_string(),
                    // bson::to_bson(&rule.write_rule).unwrap(),
                    rule.write_rule as i32,
                );
                rule_doc.insert(
                    VIEW_RULES_WRITE_FILTERS_FIELD_ID.to_string(),
                    bson::to_bson(
                        &rule
                            .write_filters
                            .iter()
                            .map(|x| *x as i32)
                            .collect::<Vec<i32>>(),
                    )
                    .unwrap(),
                );
                docs.push(rule_doc);
            }
        }

        let mut batch_docs: Vec<Document> = Vec::new();
        for mut doc in docs {
            let id = if let Some(i) = get_new_entity_id(VIEW_RULES_MANAGE_ID, root_id).await {
                i
            } else {
                log::error!("{}: {}", t!("取得新实体ID失败"), VIEW_RULES_MANAGE_ID);
                continue;
            };
            doc.insert(ID_FIELD_ID.to_string(), id.to_string());
            batch_docs.push(doc);
        }
        
        // 批量添加到数据库
        match entity::batch_insert_entities(VIEW_RULES_MANAGE_ID, &mut batch_docs, root_id, root_group_id).await {
            Ok(r) => {
                log::info!("{}: {manage_id}, {r} 条映像规则", t!("初始化视图规则成功"));
            }
            Err(e) => {
                log::error!("{}: {}", t!("初始化视图规则失败"), VIEW_RULES_MANAGE_ID);
            }
        }
    }
    
    // 初始化视图规则索引, VIEW_RULES_LEVEL_FIELD_ID, VIEW_RULES_SUBJECT_MANAGE_FIELD_ID, VIEW_RULES_SUBJECT_FIELD_ID, VIEW_RULES_GROUP_FIELD_ID
    if let Some(c) = database::get_collection_by_id(VIEW_RULES_MANAGE_ID).await {
        // 复合索引顺序影响查询效率
        let model = IndexModel::builder().keys(doc! {
            VIEW_RULES_GROUP_FIELD_ID.to_string(): 1,
            // VIEW_RULES_SUBJECT_MANAGE_FIELD_ID.to_string(): 1,
            // VIEW_RULES_LEVEL_FIELD_ID.to_string(): 1,
            // VIEW_RULES_SUBJECT_FIELD_ID.to_string(): 1,
        }).build();

        match c.create_index(model).await{
            Ok(_r) => {
                log::info!("{}: {}", t!("初始化视图规则索引成功"), VIEW_RULES_MANAGE_ID);
            },
            Err(e) => {
                log::error!("{}: {}", t!("初始化视图规则索引失败"), VIEW_RULES_MANAGE_ID);
                log::error!("{}: {}", t!("错误信息"), e);
            }
        };
    } else {
        log::error!("{}: {}", t!("获取集合失败"), VIEW_RULES_MANAGE_ID);
        log::error!("{}: {}", t!("初始化视图规则索引失败"), VIEW_RULES_MANAGE_ID);
    };
}
