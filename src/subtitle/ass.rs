use crate::subtitle::{Dialogue, Subtitle};
use anyhow::bail;
use std::fmt::{Debug, Formatter};
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct Ass {
    pub(crate) dialogues: Vec<Dialogue>,
}

impl Subtitle for Ass {
    type T = Ass;

    fn dialogues(&self) -> Vec<Dialogue> {
        self.dialogues.clone()
    }

    fn parse(text: &str) -> anyhow::Result<Self::T> {
        let lines = text.split("\n");
        let mut dialogues = vec![];
        for line in lines {
            if line.starts_with("Dialogue") {
                let dialogque = parse_event(
                    line,
                    vec![
                        "Layer", "Start", "End", "Style", "Name", "MarginL", "MarginR", "MarginV",
                        "Effect", "Text",
                    ],
                )?;
                dialogues.push(dialogque);
            }
        }
        Ok(Self::T { dialogues })
    }
}

fn parse_event(dialogue: &str, mut headers: Vec<&str>) -> anyhow::Result<Dialogue> {
    let dialogue = dialogue.replace("Dialogue:", "");
    let values: Vec<&str> = dialogue.split(",").collect();
    let mut i = 0;
    let mut start: Option<String> = None;
    let mut end: Option<String> = None;
    let mut text: Option<String> = None;
    for value in &values {
        let header = headers.get(i);
        i += 1;
        let Some(header) = header else {
            continue;
        };
        let header = *header;
        if header == "Start" {
            start = Some(value.to_string());
        } else if header == "End" {
            end = Some(value.to_string());
        }
    }

    let text_index = headers.iter().position(|&e| e == "Text");
    if let Some(index) = text_index {
        let texts = &values[index..];
        text = Some(texts.join(","))
    }

    if let (Some(start), Some(end), Some(text)) = (start, end, text) {
        Ok(Dialogue { start, end, text })
    } else {
        bail!("start, end, text is none".to_string())
    }
}
