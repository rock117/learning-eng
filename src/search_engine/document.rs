use tantivy::schema::Field;
use tantivy::TantivyDocument;

pub struct User;

impl From<TantivyDocument> for User {
    fn from(value: TantivyDocument) -> Self {
        todo!()
     //   let f = Field
       // let name = value.get_first()
    }
}