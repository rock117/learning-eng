use crate::subtitle::{Dialogue, Subtitle};
use anyhow::{anyhow, bail};
use std::fmt::format;

pub(crate) struct Srt {
    pub(crate) dialogues: Vec<Dialogue>,
}

impl Subtitle for Srt {
    type T = Srt;

    fn dialogues(&self) -> Vec<Dialogue> {
        self.dialogues.clone()
    }

    fn parse(text: &str) -> anyhow::Result<Self::T> {
        let mut lines = text.split("\n").collect::<Vec<&str>>();
        let mut dialogues = vec![];
        while !lines.is_empty() {
            dialogues.push(parse(remove_n(&mut lines, 5))?)
        }
        Ok(Self::T { dialogues })
    }
}

fn parse(dialogues: Vec<&str>) -> anyhow::Result<Dialogue> {
    if dialogues.len() < 4 {
        return bail!("dialogues format invalid");
    }
    if let (_, Some(time), Some(chn), Some(eng)) = (
        dialogues.get(0),
        dialogues.get(1),
        dialogues.get(2),
        dialogues.get(3),
    ) {
        let start_end = time.split("-->").collect::<Vec<&str>>();
        let text = format!("{}<==>{}", chn, eng);
        return Ok(Dialogue::new(
            start_end.get(0).unwrap_or(&"").to_string(),
            start_end.get(1).unwrap_or(&"").to_string(),
            text,
        ));
    };
    return bail!("dialogues format invalid");
}

fn remove_n<T>(datas: &mut Vec<T>, n: usize) -> Vec<T> {
    let n = if n > datas.len() { n } else { datas.len() };
    let mut vec = vec![];
    for _ in 0..n {
        vec.push(datas.remove(0))
    }
    vec
}
