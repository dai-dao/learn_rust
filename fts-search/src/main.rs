use serde::Deserialize;
use quick_xml::de::{DeError, from_reader};
use std::io::{self, BufReader};
use std::fs::File;
use std::time::{Duration, Instant};
use regex::Regex;
use std::collections::{HashMap, HashSet};


extern crate rust_stemmers;
use rust_stemmers::{Algorithm, Stemmer};


#[derive(Debug, Deserialize, PartialEq)]
struct Document {
    title: String,
    url: String,
    #[serde(rename = "abstract", default)]
    text: String,
    id: Option<usize>
}

#[derive(Debug, Deserialize, PartialEq)]
struct Feed {
    #[serde(rename = "doc", default)]
    documents: Vec<Document>
}

fn get_feed(bufread: BufReader<File>) -> Result<Feed, DeError> {
    let mut feed: Feed = from_reader(bufread)?;
    for (id, doc) in feed.documents.iter_mut().enumerate() {
        doc.id = Some(id);
    }
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
        if is_word(rt.to_string()) && !rt.is_empty() {
            tokens.push(rt.to_string());
        }
    }
    return tokens
}


fn lowercase_filter(tokens: Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    for rt in tokens {
        out.push(rt.to_lowercase());
    }
    return out
}


fn stopword_filter(tokens: Vec<String>) -> Vec<String> {
    let stop_words =  vec!["a", "and", "be", "have", "i", "in", "of", "that", "the", "to"];
    let mut out = Vec::new();
    for rt in tokens {
        if !stop_words.contains(&&*rt) {
            out.push(rt);
        } 
    }
    return out
}


fn stemmer_filter(tokens: Vec<String>) -> Vec<String> {
    let en_stemmer = Stemmer::create(Algorithm::English);
    let mut out = Vec::new();
    for rt in tokens {
        out.push(en_stemmer.stem(&&*rt).to_string());
    }
    return out
}


fn analyze(text: String) -> Vec<String> {
    let mut tokens = tokenize(text);
    tokens = lowercase_filter(tokens);
    tokens = stopword_filter(tokens);
    tokens = stemmer_filter(tokens);
    return tokens
}


fn make_index(docs : Vec<Document>) -> HashMap<String, HashSet<usize>> {
    let mut index : HashMap<String, HashSet<usize>> = HashMap::new();
    for doc in docs {
        let tokens = analyze(doc.text);
        for token in tokens {
            // cloning to own the value
            index.entry(token.clone()).or_insert(HashSet::new());
            index.get_mut(&token).unwrap().insert(doc.id.unwrap());
        }
    }
    return index
}


fn main() -> io::Result<()> {
    // let f = File::open("enwiki-latest-abstract1.xml")?;
    let f = File::open("small.xml")?;
    let reader = BufReader::new(f);
    let feed = get_feed(reader).unwrap();

    // let start = Instant::now();
    // let docs = search_regex(feed, "cat".to_string());
    // println!("Results {:?}", docs);
    // let duration = start.elapsed();
    // println!("Time elapsed in search: {:?}", duration);
    
    
    // let tokens = analyze("I Loves my baby !! 123 31Q !".to_string());
    // println!("Tokens are {:?}", tokens);

    let index = make_index(feed.documents);
    println!("Index {:?}", index);

    Ok(())
}