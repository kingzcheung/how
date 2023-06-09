use clap::{command, Parser};
use how::{data::{sync_data, Searcher}, text::CompileText};
use colored::*;
#[derive(Parser, Debug)]
#[command(name = "how")]
#[command(author = "kingzcheung<kingzcheung@gmail.com>")]
#[command(version = "0.1.0")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// cli name or desc
    name: String,
}

const COMMAND_URL: &str = "https://unpkg.com/linux-command/command";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let data = sync_data().await?;
    match data.search(&args.name) {
        Some(cmds) =>{
            if !cmds.is_empty() {
                println!("您要找的是不是下面这些命令:");
                println!();
                for (cmd,desc) in cmds {
                    println!("{:15}: {}",cmd,desc);
                }
            }else {
                println!("{}","未找到相关的命令".red())
            }
             
        }
        None=>{
            let url = format!("{}/{}.md", COMMAND_URL, args.name);
            let text = reqwest::get(url).await?.text().await?;
            let mdtext = markdown::to_mdast(&text, &markdown::ParseOptions::default()).unwrap();
            // dbg!(mdtext);
            let text = mdtext.to_text().join("");
            println!("{}", text);
        }
    }

    
    Ok(())
}


