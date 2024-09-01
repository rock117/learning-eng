use crate::subtitle::{Dialogue, Subtitle};
use anyhow::bail;
use std::fmt::{Debug, Formatter};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
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


pub fn extrac_subtitle(){
    let dest = r#"C:\rock\doc\english\字幕\Good Luck Charlie S01-04\extract\01"#;
    let dir = r#"C:\rock\doc\english\字幕\Good Luck Charlie S01-04\Good Luck Charlie (2010) Season 1 S01 (1080p WEB-DL x265 HEVC 10bit AAC 5.1 Panda)"#;
    let dir = std::fs::read_dir(dir).unwrap();
    for file in dir {
        let file = file.unwrap().path();
       // println!("file name: {:?}", file);
        let name = file.file_name().unwrap().to_string_lossy().to_string();
        let content = std::fs::read_to_string(file).unwrap();
        let dialogues = Ass::parse(&content).unwrap().dialogues;
        let mut target_file = BufWriter::new(OpenOptions::new().write(true).truncate(true).create(true).open(format!("{}/{}", dest, name.replace(".ass", ".txt"))).unwrap());
        let mut i = 1;
        for dialogue in dialogues {
            let ch_en = dialogue.split_en_che();
            let line = format!("{:04} {} => {}\n", i, ch_en.1, ch_en.0);
            target_file.write(line.as_bytes()).unwrap();
            i += 1;
        }
    }
}