use std::fmt::{Debug, Formatter};

pub mod ass;
pub mod srt;

pub(crate) trait Subtitle {
    type T: Subtitle;
    fn dialogues(&self) -> Vec<Dialogue>;

    fn parse(text: &str) -> anyhow::Result<Self::T>;
}

#[derive(Clone)]
pub struct Dialogue {
    start: String,
    end: String,
    text: String,
}

impl Dialogue {
    pub fn new(start: String, end: String, text: String) -> Self {
        Self { start, end, text }
    }

    pub fn split_en_che(&self) -> (String, String) {
        let text: &str = self.text.as_str();
        let ch_index = text.find("{");
        let eng_index = text.rfind("}");
        let ch = if let Some(ch_index) = ch_index {
            (&text[0..ch_index]).to_string().replace("\n", "").replace("\r", "")
        } else {
            "".into()
        };
        let eng = if let Some(eng_index) = eng_index {
            (&text[eng_index + 1..text.len()]).to_string().replace("\n", "").replace("\r", "")
        } else {
            "".into()
        };
        (ch, eng)
    }
}

impl Debug for Dialogue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "start: {:?}, end: {:?}, text: {:?}, ch_eng: {:?}",
            self.start,
            self.end,
            self.text,
            self.split_en_che()
        )
    }
}
