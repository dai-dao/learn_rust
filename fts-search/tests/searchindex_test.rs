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
    let fts = make_index(feed.documents);
    let res = fts.search_phrase_index("solar political radiation reflection philosophy movement".to_string());
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
    let fts = make_index(feed.documents);
    let res = fts.search_bool_index("solar OR movement".to_string());
    assert!(res.len() == 0);
    //
    Ok(())
}