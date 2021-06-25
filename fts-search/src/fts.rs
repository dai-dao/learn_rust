use serde::Deserialize;
use quick_xml::de::{DeError, from_reader};
use std::io::BufReader;
use std::fs::File;
use std::collections::{HashMap, HashSet};
use crate::boolparser::*;


extern crate rust_stemmers;
use rust_stemmers::{Algorithm, Stemmer};


type Index = HashMap<String, HashSet<usize>>;


#[derive(Debug)]
pub struct FTS {
    pub index: Index,
    pub all_ids: HashSet<usize>
}

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

impl FTS {

    fn search_bool(&self, node: ASTNode) -> HashSet<usize> {
        match node {
            ASTNode::Binary(BinaryOp::And, left, right) => {   
                return self.search_bool(*left).intersection(&self.search_bool(*right)).copied().collect()
            }
            ASTNode::Binary(BinaryOp::Or, left, right) => {
                return self.search_bool(*left).union(&self.search_bool(*right)).copied().collect()
            }
            ASTNode::Invert(node) => {
                let ids = self.search_bool(*node);
                return self.all_ids.difference(&ids).copied().collect();
            }
            ASTNode::Name(name) => {
                let token = &analyze(name)[0];
                match &self.index.get(token) {
                    Some(ids) => {
                        return HashSet::new().union(ids).copied().collect()
                    }
                    None => return HashSet::new()
                }
            }
        }
    }
    
    // boolean search
    pub fn search_bool_index(&self, text: String) -> HashSet<usize> {
        let tokens = lex(text);
        let ast = munch_tokens(tokens);
        // recurse AST tree and build the output postings
        match ast {
            Some(tbox) => return self.search_bool(*tbox),
            _ => return HashSet::new()
        }
    }

    // intersection search, need to match all tokens in a phrase
    pub fn search_phrase_index(&self, text: String) -> HashSet<usize> {
        let mut out : HashSet<_> = HashSet::new();
        for token in analyze(text) {
            match self.index.get(&token) {
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
}

pub fn make_index(docs : Vec<Document>) -> FTS {
    let mut index : Index = HashMap::new();
    let mut all_ids = HashSet::new();
    for doc in docs {
        all_ids.insert(doc.id.unwrap());
        let tokens = analyze(doc.text);
        for token in tokens {
            // cloning to own the value
            index.entry(token.clone()).or_insert(HashSet::new());
            index.get_mut(&token).unwrap().insert(doc.id.unwrap());
        }
    }
    let fts = FTS { index, all_ids };
    return fts
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
