// https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/

use std::collections::HashMap;


struct Todo {
    map: HashMap<String, bool>
}


impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .read(true)
                        .open("db.json")?;
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo {map}),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }
    fn insert(&mut self, key: &String) {
        self.map.insert(key.to_string(), true);
    }
    // take ownership of self to ensure this is the last 
    // operation, and that map can not be updated after calling save
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open("db.json")?;
        //
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }
    // Result will help return an error, option just returns None
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}


fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");
    //
    let mut todo = Todo::new().expect("Initialisation of db faild");
    if action == "add" {
        todo.insert(&item); // pass reference to not give ownership yet
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(e) => println!("An error occurred: {}", e),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo saved"),
                Err(e) => println!("An error occurred: {}", e),
            }
        }
    }

    println!("{:?}, {:?}", action, item);
}
