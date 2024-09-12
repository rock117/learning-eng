use std::collections::HashMap;
use tantivy::schema::Schema;
use tantivy::DocAddress;

pub struct IndexSetting {
    dir: String,
    schema: Schema,
}

pub fn update(address: &DocAddress, fields: HashMap<String, String>) {}
pub fn insert(address: &DocAddress, fields: HashMap<String, String>) {}

pub fn delete(address: &DocAddress) {}

pub fn search(keyword: &str) {}
