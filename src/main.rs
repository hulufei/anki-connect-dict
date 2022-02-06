use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use reqwest::{blocking::Response, Error};
use serde::Deserialize;
use std::{
    fs::{create_dir, read_to_string, rename},
    path::Path,
    sync::mpsc::channel,
    time::Duration,
};

const ANKI_SERVER: &'static str = "http://127.0.0.1:8765";

fn gui_add_cards(front: &str, back: &str) -> Result<Response, Error> {
    let client = reqwest::blocking::Client::new();
    let body = r#"
{
    "action": "guiAddCards",
    "version": 6,
    "params": {
        "note": {
            "deckName": "Default",
            "modelName": "Basic",
            "fields": {
                "Front": "{front}",
                "Back": "{back}"
            },
            "options": {
                "closeAfterAdding": true
            }
        }
    }
}
    "#;
    client
        .post(ANKI_SERVER)
        .body(body.replace("{front}", front).replace("{back}", back))
        .send()
}

fn get_word_back<P: AsRef<Path>>(path: P) -> Option<String> {
    let content = read_to_string(path).unwrap();
    let mut partitions = content.split("<body>");
    partitions = partitions.nth(1).unwrap().split("</body>");

    partitions.next().map(String::from)
}

#[derive(Deserialize)]
struct AnkiResult {
    error: Option<String>,
}

fn handle_word_file(path: &Path, enable_archive: bool) {
    let filename = path.file_name().unwrap().to_string_lossy();
    let mut partitions = filename.split('.');
    let front = partitions.next().unwrap();
    let back = get_word_back(path).expect("Get word's back failed");
    let back = json::stringify(back);
    // let back = json::stringify(r#"<p class="p1">test</p>"#);
    let back = back.trim_matches('"');
    // println!("front {front}, back {back}");
    match gui_add_cards(front, &back)
        .and_then(|res| res.json::<AnkiResult>())
        .map(|r| r.error)
    {
        Ok(None) => {
            if !enable_archive {
                return;
            }
            // Move to imported directory
            let to = path.with_file_name(format!("imported/{filename}"));
            to.parent().map(create_dir);
            match rename(path, to) {
                Ok(_) => println!("{filename} archived to imported/"),
                Err(e) => println!("Archive {filename} failed: {:?}", e),
            }
        }
        Ok(Some(e)) => println!("Anki error {:#?}", e),
        Err(e) => println!("{:#?}", e),
    }
}

fn main() {
    let mut args = std::env::args();
    let file_path = args.nth(1).expect("Html word file required");
    let path = Path::new(&file_path);
    if path.is_dir() {
        // Watch vocabulary directory
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();
        watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
        println!("Watching {:?}", path);
        loop {
            match rx.recv() {
                Ok(DebouncedEvent::Create(p) | DebouncedEvent::NoticeRemove(p)) if p.is_file() => {
                    handle_word_file(&p, true);
                }
                Err(e) => println!("Watch error: {:?}", e),
                Ok(e) => println!("Notify event: {:?}", e),
            }
        }
    } else {
        handle_word_file(path, false);
    }
}
