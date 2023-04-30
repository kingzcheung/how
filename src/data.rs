use std::{collections::HashMap, io::Write};

use serde::Deserialize;

const DATA_INDEX_URL: &str = "https://unpkg.com/linux-command/dist/data.json";

pub trait Searcher {
    fn search(&self, s: &str) -> Option<Vec<(String,String)>>;
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub n: String,
    pub p: String,
    pub d: String,
}

pub type Data = HashMap<String, Item>;

pub async fn sync_data() -> anyhow::Result<Data> {
    let mut data_path = home::home_dir().unwrap_or("~".into());
    data_path.push(".how/data.json");

    match data_path.metadata() {
        Ok(meta) => {
            if !meta.is_file() {
                eprintln!("{:?} 不是文件", data_path.as_path());
            }
            let b = std::fs::read(data_path)?;
            let data: Data = serde_json::from_slice(&b)?;

            Ok(data)
        }
        Err(_) => {
            let res = reqwest::get(DATA_INDEX_URL)
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let parent = data_path.parent().unwrap();
            if !parent.exists() {
                // println!("create dir: {:?}", &parent);
                std::fs::create_dir(parent)?;
            }

            let mut file = std::fs::File::create(data_path)?;
            file.write_all(res.as_bytes())?;

            let data: Data = serde_json::from_slice(res.as_bytes())?;

            Ok(data)
        }
    }
}

impl Searcher for Data {
    fn search(&self, s: &str) -> Option<Vec<(String,String)>> {
        let mut commands = vec![];
        let mut search_result = false;
        for (k, v) in self {
            if k == s {
                search_result = true;
                break;
            }
            if v.d.contains(s) {
                commands.push((v.n.clone(),v.d.clone()));
            }
        }
        if search_result {
            None
        } else {
            Some(commands)
        }
    }
}
