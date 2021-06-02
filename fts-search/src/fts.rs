use serde::Deserialize;
use quick_xml::de::{DeError, from_reader};
use std::io::BufReader;
use std::fs::File;
use std::collections::{HashMap, HashSet};


extern crate rust_stemmers;
use rust_stemmers::{Algorithm, Stemmer};


type Index = HashMap<String, HashSet<usize>>;


#[derive(Debug, Deserialize, PartialEq)]
pub struct Document {
    title: String,
    url: String,
    #[serde(rename = "abstract", default)]
    text: String,
    id: Option<usize>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Feed {
    #[serde(rename = "doc", default)]
    pub documents: Vec<Document>
}

pub fn get_feed(bufread: BufReader<File>) -> Result<Feed, DeError> {
    let mut feed: Feed = from_reader(bufread)?;
    for (id, doc) in feed.documents.iter_mut().enumerate() {
        doc.id = Some(id);
    }
    Ok(feed)
}

// fn search(feed: Feed, term: String) -> Vec<Document> {
//     let mut out = Vec::new();
//     for doc in feed.documents {
//         if doc.text.contains(&term) {
//             out.push(doc);
//         }
//     }
//     out
// }

// intersection search, need to match all tokens
pub fn search_index(index: &Index, text: String) -> HashSet<usize> {
    let mut out : HashSet<_> = HashSet::new();
    for token in analyze(text) {
        match index.get(&token) {
            Some(ids) => {
                if out.len() == 0 {
                    out = out.union(ids).copied().collect();
                } else {
                    out = out.intersection(ids).copied().collect();
                }
            }
            None => ()
        }
    }
    return out
}

// fn is_numeric(word: String) -> bool {
//     for w in word.chars() {
//         if !w.is_numeric() {
//             return false;
//         }
//     }
//     return true;
// }

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


pub fn make_index(docs : Vec<Document>) -> Index {
    let mut index : Index = HashMap::new();
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
