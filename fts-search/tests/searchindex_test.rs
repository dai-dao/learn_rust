use std::io::{self, BufReader};
use std::fs::File;
pub use fts_search::boolparser::*;
pub use fts_search::fts::*;



#[test]
fn test_search() -> io::Result<()>  {
    let f = File::open("small.xml")?;
    let reader = BufReader::new(f);
    let feed = get_feed(reader).unwrap();
    // 
    let index = make_index(feed.documents);
    let res = search_phrase_index(&index, "solar political radiation reflection philosophy movement".to_string());
    assert!(res.len() > 0);
    //
    Ok(())
}


#[test]
fn test_bool_search() -> io::Result<()> {
    let f = File::open("small.xml")?;
    let reader = BufReader::new(f);
    let feed = get_feed(reader).unwrap();
    // 
    let index = make_index(feed.documents);
    let res = search_bool_index(&index, "solar OR movement".to_string());
    assert!(res.len() == 0);
    //
    Ok(())
}