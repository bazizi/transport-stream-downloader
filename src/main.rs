use std::{fs, io::Write};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to a file containing web urls for all segments of the web content
    /// (the segments are expected to be the samw order as they appear in the original file)
    #[arg(short, long)]
    urls_file: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let client = std::sync::Arc::new(std::sync::Mutex::new(reqwest::Client::new()));
    let urls = fs::read_to_string(args.urls_file)
        .unwrap()
        .lines()
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut handles = Vec::new();

    let mut index = 0;
    for url in urls.clone() {
        let url = url.clone();
        let client = client.clone();
        handles.push(tokio::task::spawn(async move {
            println!("Downloading: {}", url);
            let bytes = client.lock().unwrap().get(&url).send();
            let bytes = bytes.await.unwrap();
            println!("Downloaded: {}", url);
            (index, bytes.bytes().await.unwrap())
        }));
        index += 1;
    }

    let mut f = fs::File::create("output.mp4").unwrap();
    for handle in handles {
        let (index, bytes) = handle.await.unwrap();
        println!("Saving: {}", urls[index]);
        f.write_all(&bytes).unwrap();
    }
}
