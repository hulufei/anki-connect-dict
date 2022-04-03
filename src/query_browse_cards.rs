use reqwest::{blocking::Response, Error};
use serde::Deserialize;

use crate::word_frequency::contain;

pub const ANKI_SERVER: &'static str = "http://127.0.0.1:8765";

#[derive(Deserialize)]
struct AnkiCardListResult {
    error: Option<String>,
    result: Vec<usize>,
}

#[derive(Deserialize)]
struct FieldItem {
    value: String,
}

#[derive(Deserialize)]
struct Field {
    #[serde(rename(deserialize = "Front"))]
    front: FieldItem,
}

#[derive(Deserialize)]
struct Card {
    fields: Field,
}

#[derive(Deserialize)]
struct AnkiCardResult {
    error: Option<String>,
    result: Vec<Card>,
}

fn gui_browse(query: &str) -> Result<Response, Error> {
    let client = reqwest::blocking::Client::new();
    let body = r#"
{
    "action": "guiBrowse",
    "version": 6,
    "params": {
        "query": "{query}"
    }
}
    "#;
    client
        .post(ANKI_SERVER)
        .body(body.replace("{query}", query))
        .send()
}

fn find_current_deck_cards() -> Result<Response, Error> {
    let client = reqwest::blocking::Client::new();
    let body = r#"
{
    "action": "findCards",
    "version": 6,
    "params": {
        "query": "deck:current"
    }
}
    "#;
    client.post(ANKI_SERVER).body(body).send()
}

fn get_cards_info(cards: &[usize]) -> Result<Response, Error> {
    let client = reqwest::blocking::Client::new();
    let body = r#"
{
    "action": "cardsInfo",
    "version": 6,
    "params": {
        "cards": {cards}
    }
}
    "#;
    client
        .post(ANKI_SERVER)
        .body(body.replace("{cards}", &serde_json::to_string(cards).unwrap()))
        .send()
}

pub fn open_browse() {
    match find_current_deck_cards()
        .and_then(|res| res.json::<AnkiCardListResult>())
        .map(|r| (r.result, r.error))
    {
        Ok((cards, None)) => {
            match get_cards_info(&cards)
                .and_then(|res| res.json::<AnkiCardResult>())
                .map(|r| (r.result, r.error))
            {
                Ok((card_list, None)) => {
                    let query = card_list
                        .into_iter()
                        .map(|item| item.fields.front.value)
                        .filter(|word| contain(&word))
                        .map(|word| format!("front:{}", word))
                        .collect::<Vec<String>>()
                        .join(" or ");
                    gui_browse(&query).unwrap();
                }
                Ok((_, Some(e))) => println!("Anki error {:#?}", e),
                Err(e) => println!("{:#?}", e),
            }
        }
        Ok((_, Some(e))) => println!("Anki error {:#?}", e),
        Err(e) => println!("{:#?}", e),
    }
}
