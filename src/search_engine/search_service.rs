use chrono::format::Item;
use crate::subtitle::Dialogue;


trait Search {
    type Item;

    fn search(keyword: &str) -> anyhow::Result<Vec<Self::Item>>;
}


pub struct SearcherReader {
    dir: String,

}

impl SearcherReader {
    pub fn new() {

    }

    pub fn search(keyword: &str) -> anyhow::Result<Vec<Dialogue>>{
        todo!()
    }
}