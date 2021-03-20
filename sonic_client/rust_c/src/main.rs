use sonic_channel::*;
use rocksdb::{DB, IteratorMode};
use std::str;


// 

fn main() -> result::Result<()> {
    // let channel = IngestChannel::start("::1:1491", "SecretPassword")?;
    // let pushed = channel.push("collection", "bucket", "object:1", "my really new good recipe")?;
    // dbg!(pushed);
    // channel.quit()?;

    // let bucket_count = channel.bucket_count("collection")?;
    // println!("{}", bucket_count);

    // let obj_count = channel.object_count("collection", "bucket");
    // println!("{:?}", obj_count);

    // let word_count = channel.word_count("collection", "bucket", "object:1");
    // println!("{:?}", word_count);

    // channel.ping()?;

////////
    let search_channel = SearchChannel::start("::1:1491", "SecretPassword")?;
//    // now i get the object id as a result, how to get the actual object content?
    let result = search_channel.query("collection", "bucket", "recipe")?;
    // search_channel.quit()?;

//    // autcompletes words, i thought it auto-suggest the whole sentence?
//    // maybe have to do with how many objects i have
    // let result = search_channel.suggest("collection", "bucket", "rec")?;

    println!("{:?}", result);

//    // query all or count?
//    // get the actual object content?

    // let path = "./data/store/kv/f09baad9";
    // let path = "/home/dai/Desktop/learn_rust/sonic/target/release/data/store/kv/f09baad9";
    // println!("Querying rocks db");

    // {
    //     let db = DB::open_default(path).unwrap();
        // db.put(b"my key", b"my value").unwrap();
        // match db.get("recipe") {
        //     Ok(Some(value)) => println!("Retrieved value {}", str::from_utf8(&value).unwrap()),
        //     Ok(None) => println!("value not found"),
        //     Err(e) => println!("operational problem occurred: {}", e),
        // }
        // let mut iter = db.iterator(IteratorMode::Start);
        // for (key, value) in iter {
        //     // println!("Saw {:?} {:?}", String::from_utf8(key.to_vec()).unwrap(), String::from_utf8(value.to_vec()).unwrap());
        //     let strValue = String::from_utf8(value.to_vec()).unwrap();
        //     if strValue == "object:1" {
        //         println!("Saw key {:?}", key);
        //         println!("Saw {:?} {:?}", String::from_utf8(key.to_vec()).unwrap(), strValue);
        //     } else {
        //         println!("Saw {:?} {:?}", key, strValue);
        //     }
            // println!("Saw {:?} {:?}", key, value);
            // println!("Saw {:?} {:?}", String::from_utf8(key.to_vec()).unwrap(), value);
            // 
            // if str::from_utf8(&key.to_vec()).is_err() {
            //     println!("error");
            // } else {
            //     println!("Saw {:?} {:?}", str::from_utf8(&key.to_vec()).unwrap(), str::from_utf8(&value.to_vec()).unwrap());
            // }
        // }
    // }

    Ok(())
}