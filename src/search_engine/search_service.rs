use crate::subtitle::Dialogue;
use chrono::format::Item;

trait Search {
    type Item;

    fn search(keyword: &str) -> anyhow::Result<Vec<Self::Item>>;
}

pub struct SearcherReader {
    dir: String,
}

impl SearcherReader {
    pub fn new() {}

    pub fn search(keyword: &str) -> anyhow::Result<Vec<Dialogue>> {
        todo!()
    }
}
