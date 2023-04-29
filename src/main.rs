use std::io::Write;

use anyhow::Result;
use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(name = "how")]
#[command(author = "kingzcheung<kingzcheung@gmail.com>")]
#[command(version = "0.1.0")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// cli name or desc
    name: String,
}

const DATA_INDEX_URL: &str = "https://unpkg.com/linux-command/dist/data.json";
const COMMAND_URL: &str = "https://unpkg.com/linux-command/command";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    sync_data().await?;


    let url = format!("{}/{}.md", COMMAND_URL, args.name);
    let text = reqwest::get(url).await?.text().await?;
    let _mdtext = markdown::to_mdast(&text, &markdown::ParseOptions::default()).unwrap();
    // dbg!(mdtext);
    println!("{}", text);
    Ok(())
}



async fn sync_data() -> Result<()> {
    let mut data_path = home::home_dir().unwrap_or("~".into());
    data_path.push(".how/data.json");

    match data_path.metadata() {
        Ok(meta) => {
            if !meta.is_file() {
                eprintln!("{:?} 不是文件", data_path.as_path());
            }
            Ok(())
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
            Ok(())
        }
    }
}
