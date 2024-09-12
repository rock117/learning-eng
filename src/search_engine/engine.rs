use anyhow::anyhow;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Schema};
use tantivy::{DocAddress, Document, Index, IndexReader, IndexWriter, ReloadPolicy, Score, TantivyDocument};

pub struct Engine {
    index: Index,
    index_writer: IndexWriter,
    index_reader: IndexReader,
}

impl Engine {

    pub fn new(dir: &str, schema: Schema) -> anyhow::Result<Self> {
        let index = Index::create_in_dir(dir, schema)?;
        let index_writer: IndexWriter = index.writer(50_000_000)?;

        let index_reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;
        Ok({
            Engine {
                index,
                index_writer,
                index_reader,
            }
        })
    }

    pub fn add_document<S: Into<TantivyDocument>>(&mut self, doc: S) -> anyhow::Result<()> {
        self.index_writer.add_document(doc.into())?;
        Ok(())
    }

    pub fn query<S: From<TantivyDocument>>(
        &self,
        query: &str,
        default_fields: Vec<&str>,
        limit: usize,
    ) -> anyhow::Result<Vec<S>> {
        let default_fields = default_fields
            .into_iter()
            .map(|field| self.index.schema().get_field(field))
            .collect::<tantivy::Result<Vec<Field>>>()?;
        let searcher = self.index_reader.searcher();
        let query_parser = QueryParser::for_index(&self.index, default_fields);
        let query = query_parser.parse_query(query)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;
        let mut datas = vec![];
        for (_score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
            datas.push(S::from(retrieved_doc));
        }
        Ok(datas)
    }
}
