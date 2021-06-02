use std::io::{self, BufReader};
use std::fs::File;
use std::time::Instant;


pub use fts_search::fts::*;



fn main() -> io::Result<()> {
    // let f = File::open("enwiki-latest-abstract1.xml")?;
    let f = File::open("small.xml")?;
    let reader = BufReader::new(f);
    let feed = get_feed(reader).unwrap();
    // 
    let index = make_index(feed.documents);
    let start = Instant::now();
    let res = search_index(&index, "Small wild cat".to_string());
    let duration = start.elapsed();
    println!("Time elapsed in search: {:?}", duration);
    println!("Search results {:?}", res);

    Ok(())
}