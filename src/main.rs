mod subtitle;

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::Index;
use crate::subtitle::ass::Ass;
use crate::subtitle::srt::Srt;
use crate::subtitle::Subtitle;

fn main() -> io::Result<()> {
    //Specify the path to the file you want to read from
    let filename = r#"C:\rock\doc\english\字幕\Modern Family S01-08\摩登家庭第一季Modern Family Season 01 S01 1080p Bluray 10bit AAC 5.1 x265 HEVC-LION[UTR]\Modern.Family.S01.E02.1080p.Bluray.AAC.5.1.x265-LION.ass"#;
   // let filename = r#"C:\rock\doc\english\字幕\★《绝望主妇》Desperate Housewives (2004-2011)[1080P]美国（共8季）【完全版】\《绝望主妇 第1季》 Desperate Housewives Season 1 (2004)[1080P]美国（共23集）\Desperate.Housewives.S01E01.1080p.WEB-DL.DD+.5.1.x264-TrollHD.srt"#;
    let text = std::fs::read_to_string(filename).unwrap();
    let ass = Ass::parse(text.as_str());
    for d in ass.unwrap().dialogues {
        println!("{:?}", d.split_en_che());
        break;
    }

    Ok(())
}


// let gbk_bytes = vec![0xC4, 0xE3, 0xBA, 0xC3]; // 这是 "中文" 的 GBK 编码
//
// // 使用 GBK 解码器来解码字节
// let decoded_str = GBK.decode(&gbk_bytes).unwrap();