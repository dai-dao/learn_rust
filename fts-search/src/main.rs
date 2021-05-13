use serde::Deserialize;
use quick_xml::de::{DeError, from_reader};
use std::io::{self, BufReader};
use std::fs::File;
use std::time::{Duration, Instant};
use regex::Regex;


#[derive(Debug, Deserialize, PartialEq)]
struct Document {
    title: String,
    url: String,
    #[serde(rename = "abstract", default)]
    text: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Feed {
    #[serde(rename = "doc", default)]
    documents: Vec<Document>
}

fn get_feed(bufread: BufReader<File>) -> Result<Feed, DeError> {
    let feed: Feed = from_reader(bufread)?;
    Ok(feed)
}

fn search(feed: Feed, term: String) -> Vec<Document> {
    let mut out = Vec::new();
    for doc in feed.documents {
        if doc.text.contains(&term) {
            out.push(doc);
        }
    }
    out
}

fn search_regex(feed: Feed, term: String) -> Vec<Document> {
    let re = Regex::new(r"(?i)\bcat\b").unwrap();
    let mut out = Vec::new();
    for doc in feed.documents {
        if re.is_match(&doc.text) {
            out.push(doc);
        }
    }
    out
}

fn is_numeric(word: String) -> bool {
    for w in word.chars() {
        if !w.is_numeric() {
            return false;
        }
    }
    return true;
}

fn is_word(word: String) -> bool {
    for w in word.chars() {
        if !w.is_alphabetic() {
            return false;
        }
    }
    return true;
}

fn tokenize(text: String) -> Vec<String> {
    // Split on any characters that is not a letter or a number
    let raw_tokens : Vec<&str> = text.split(" ").collect();
    let mut tokens = Vec::new();
    for rt in raw_tokens {
        if is_word(rt.to_string()) && !is_numeric(rt.to_string()) {
            tokens.push(rt.to_lowercase().to_string());
        }
    }
    return tokens
}


fn main() -> io::Result<()> {
    // let f = File::open("enwiki-latest-abstract1.xml")?;
    // let reader = BufReader::new(f);
    // let feed = get_feed(reader).unwrap();

    // let start = Instant::now();
    // let docs = search_regex(feed, "cat".to_string());
    // println!("Results {:?}", docs);
    // let duration = start.elapsed();
    // println!("Time elapsed in search: {:?}", duration);
    let tokens = tokenize("I love my baby !! 123 31Q !".to_string());
    println!("Tokens are {:?}", tokens);

    Ok(())
}