use dependencies_sync::bson::{doc, Document};
use dependencies_sync::futures::StreamExt;
use dependencies_sync::log::{error};
use dependencies_sync::mongodb::{Database, IndexModel};
use dependencies_sync::rust_i18n::{self, t};

use manage_define::general_field_ids::{
    ID_FIELD_ID, JIAN_PIN_FIELD_ID, MODIFY_TIMESTAMP_FIELD_ID, OWNER_FIELD_ID, QUAN_PIN_FIELD_ID,
};

pub async fn init_indexes(db: &Database, manage_id: &str, indexes_doc: &[Document]) {
    // 取得collection
    let collection = db.collection::<Document>(manage_id);

    // 一般索引
    let mut index_docs: Vec<Document> = [
        doc! {ID_FIELD_ID.to_string(): 1},
        doc! {MODIFY_TIMESTAMP_FIELD_ID.to_string(): -1},
        doc! {OWNER_FIELD_ID.to_string(): 1},
        doc! {QUAN_PIN_FIELD_ID.to_string(): 1},
        doc! {JIAN_PIN_FIELD_ID.to_string(): 1},
    ].to_vec();

    index_docs.extend(indexes_doc.iter().cloned());

    let indexes = index_docs
        .iter()
        .map(|d| IndexModel::builder().keys(d.clone()).build());

    if let Err(err) = collection.create_indexes(indexes).await{
        error!("{}: {}, {:?}", t!("创建索引失败"), manage_id, err);
    };
}
