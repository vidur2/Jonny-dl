// extern crate reqwest;

use tokio;
use youtube_dl::{YoutubeDl, YoutubeDlOutput};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io;


#[derive(Debug, Deserialize, Serialize)]
struct YtdlWrapper {
    youtube_dl: YoutubeDlOutput
}
async fn download(url: String, path: String) {
    let resp = reqwest::get(url).await.unwrap();
    let bytes = resp.bytes().await.unwrap();
    let mut slice: &[u8] = bytes.as_ref();
    let mut out = File::create(path).expect("failed to create file");
    io::copy(&mut slice, &mut out).expect("failed to copy content");
}

#[tokio::main]
async fn main() {
    let mut input = String::new();
    println!("Link: ");
    std::io::stdin().read_line(&mut input).expect("Could not read line properly");
    println!("Path: ");
    let mut path = String::new();
    std::io::stdin().read_line(&mut path).expect("Could not read line properly");
    let output = YoutubeDl::new(input.as_str())
        .extract_audio(true)
        .run()
        .unwrap();
    let wrapper = YtdlWrapper {
        youtube_dl: output
    };
    let serialized_output = serde_json::to_string(&wrapper).expect("Could not convert");
    let parts: Vec<&str> = serialized_output.split("url").collect();
    let final_url: Vec<&str> = parts[parts.len() - 2].split("\"").collect();
    download(String::from(final_url[2]), path).await;

}

