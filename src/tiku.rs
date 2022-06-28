
use rust_fuzzy_search::fuzzy_compare;
use office::{DataType, Excel};
use std::path::Path;

#[derive(Clone, Debug)]
pub enum 题目类型 {
    单选题 { content: String, answer: String },
    多选题 { content: String, answer: String },
    判断题 { content: String, answer: bool },
    填空题 { content: String, answer: String },
}

pub fn fuzzy_find(tiku: &Vec<题目类型>, timu: &str, typ: &str) -> (f32, Option<题目类型>) {
    let mut score = 0.0;
    let mut ti: Option<题目类型> = None;
    for item in tiku.iter() {
        let content = match item {
            题目类型::单选题 { content, answer: _ } => if typ.contains("单选题") { content } else { continue },
            题目类型::多选题 { content, answer: _ } => if typ.contains("多选题") { content } else { continue },
            题目类型::判断题 { content, answer: _ } => if typ.contains("判断题") { content } else { continue },
            题目类型::填空题 { content, answer: _ } => if typ.contains("填空题") || typ.contains("论述题") { content } else { continue },
        };
        let s = fuzzy_compare(timu, content);
        if s > score {
            score = s;
            ti = Some(item.clone())
        }
    }
    (score, ti)
}

pub fn load(path: impl AsRef<Path>) -> Vec<题目类型> {
    let mut excel = Excel::open(path).unwrap();
    let r = excel.worksheet_range("Sheet1").unwrap();
    let mut list = vec![];
    for row in r.rows().skip(1) {
        let content = if let DataType::String(content) = row[0].clone() {
            content.replace("_x000D_", "").replace("\r\n", "\n").trim().to_owned()
        } else {
            unimplemented!()
        };
        let answer = if let DataType::String(answer) = row[1].clone() {
            answer.replace("_x000D_", "").replace("\r\n", "\n").trim().to_owned()
        } else {
            unimplemented!()
        };
        let typ = if let DataType::String(typ) = row[2].clone() {
            typ.trim().to_owned()
        } else {
            unimplemented!()
        };

        list.push(match typ.as_str() {
            "单选题" => 题目类型::单选题 { content, answer },
            "多选题" => 题目类型::多选题 { content, answer },
            "判断题" => 题目类型::判断题 {
                content,
                answer: if answer.contains("对") {
                    true
                } else if answer.contains("正确") {
                    true
                } else {
                    false
                },
            },
            "填空题" => 题目类型::填空题 { content, answer },
            "论述题" => 题目类型::填空题 { content, answer },
            _ => unimplemented!(),
        });
    }
    list
}