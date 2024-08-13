use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::opts::OutputFormat;
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}

#[allow(unused_variables)]
pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // headers.iter() -> 使用headers的迭代器
        // record.iter() -> 使用record的迭代器
        // zip() -> 将使用迭代器合并为一个元祖的迭代器 [(header, record), ...]
        // collect::<Value>() -> 将迭代器转换为json_value
        // 函数式编程
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}
